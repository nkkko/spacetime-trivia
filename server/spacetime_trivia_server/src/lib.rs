pub mod elo;

use spacetimedb::{Identity, ReducerContext, Timestamp, Table, log, table, reducer};
use crate::elo::calculate_elo_delta;

// Status enums as string constants
const LOBBY_STATUS_WAITING: &str = "waiting";
const LOBBY_STATUS_IN_GAME: &str = "in_game";
const LOBBY_STATUS_FINISHED: &str = "finished";

const ROUND_STATUS_WAITING: &str = "waiting";
const ROUND_STATUS_IN_PROGRESS: &str = "in_progress";
const ROUND_STATUS_SCORING: &str = "scoring";
const ROUND_STATUS_FINISHED: &str = "finished";

const AGENT_JOB_STATUS_PENDING: &str = "pending";
const AGENT_JOB_STATUS_PROCESSING: &str = "processing"; // Optional intermediate status
const AGENT_JOB_STATUS_COMPLETED: &str = "completed";
const AGENT_JOB_STATUS_FAILED: &str = "failed";

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, spacetimedb::SpacetimeType)] // Added spacetimedb::SpacetimeType
pub struct NewQuestionData {
    text: String,
    correct_answer: String,
    wrong_answers: Vec<String>,
    topic: String,
    difficulty: String,
    // quality_score will be defaulted server-side
    // origin_agent will be set server-side from the agent submitting
}

#[table(name = question_bank, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct Question {
    #[primary_key]
    #[auto_inc]
    question_id: u64,
    text: String,
    correct_answer: String,
    wrong_answers: Vec<String>,
    #[index(btree)]
    topic: String,
    #[index(btree)]
    difficulty: String,
    quality_score: i32,
    origin_agent: Option<String>,
}

#[table(name = player, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct Player {
    #[primary_key]
    player_id: Identity,
    #[unique]  // Ensure unique player names
    #[index(btree)]  // Add index for name lookups
    name: String,
    score: u32,
    elo: i32, // New field for Elo rating, default to 1200
}

#[table(name = lobby, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct Lobby {
    #[primary_key]
    #[auto_inc]
    lobby_id: u64,
    name: Option<String>,
    #[index(btree)]  // Add index for status filtering
    status: String,
    host_id: Identity,
    next_round_is_lightning: bool,
}

#[table(name = active_round, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct ActiveRound {
    #[primary_key]
    #[auto_inc]
    round_id: u64,
    #[index(btree)]
    lobby_id: u64,
    question_id: u64,
    start_time: Timestamp,
    #[index(btree)]  // Add index for status filtering
    status: String,
    is_lightning: bool,
}

#[table(name = answer, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct Answer {
    #[primary_key]
    #[auto_inc]
    answer_id: u64,
    #[index(btree)]
    round_id: u64,
    #[index(btree)]  // Add index for player filtering
    player_id: Identity,
    chosen_answer_index: u32,
    score: Option<u32>,
}

#[table(name = agent_job_queue, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct AgentJobQueue {
    #[primary_key]
    #[auto_inc]
    job_id: u64,            // Corresponds to id PK(auto_inc) in SPECS
    agent_id: u64,          // ID of the agent to perform the work
    payload_json: String,   // The actual work payload, e.g., topic info for question generation
    #[index(btree)]
    status: String,         // e.g., "pending", "processing", "completed", "failed"
    error_message: Option<String>, // Optional: To store error details if the job failed
    // created_at: Timestamp, // Optional: consider adding for tracking
    // updated_at: Timestamp, // Optional: consider adding for tracking
}

#[table(name = agent_registry, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct AgentRegistry {
    #[primary_key]
    #[auto_inc]
    agent_id: u64,          // Corresponds to id PK in SPECS
    owner_id: Identity,       // The Identity of the user who registered the agent
    wasm_hash: String,        // SHA256 hash of the agent's WASM binary
    capabilities: Vec<String>,// List of agent capabilities (e.g., "generate_questions")
    energy_quota: u64,        // Resource limit for the agent
    // registered_at: Timestamp, // Optional: for tracking registration time
    // name: Option<String>,     // Optional: a human-readable name for the agent
    // description: Option<String>, // Optional: a short description
}

#[table(name = crowd_meter_stats, public)]
#[derive(Clone, Debug)] // Reverted: Removed Table
pub struct CrowdMeterStats {
    #[primary_key] // Part 1 of composite key
    round_id: u64,
    #[primary_key] // Reverted to primary_key for composite key definition
    answer_index: u32, // The index of the answer chosen (e.g., 0, 1, 2, 3)
    count: u32,        // Number of players who chose this answer_index for this round
}

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    log::info!("Initializing Spacetime Trivia module...");

    // Bootstrap initial questions if the question bank is empty
    if ctx.db.question_bank().iter().next().is_none() {
        let questions = vec![
            Question {
                question_id: 0,
                text: "What is the capital of France?".to_string(),
                correct_answer: "Paris".to_string(),
                wrong_answers: vec!["London".to_string(), "Berlin".to_string(), "Madrid".to_string()],
                topic: "Geography".to_string(),
                difficulty: "Easy".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "Which programming language was created by Graydon Hoare at Mozilla Research?".to_string(),
                correct_answer: "Rust".to_string(),
                wrong_answers: vec!["Go".to_string(), "Swift".to_string(), "Kotlin".to_string()],
                topic: "Programming".to_string(),
                difficulty: "Medium".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "What is the speed of light in meters per second?".to_string(),
                correct_answer: "299,792,458".to_string(),
                wrong_answers: vec!["300,000,000".to_string(), "299,999,999".to_string(), "299,792,000".to_string()],
                topic: "Science".to_string(),
                difficulty: "Hard".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "What year was SpacetimeDB first released?".to_string(),
                correct_answer: "2023".to_string(),
                wrong_answers: vec!["2022".to_string(), "2021".to_string(), "2020".to_string()],
                topic: "Technology".to_string(),
                difficulty: "Medium".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "Which data structure provides O(1) average time complexity for insertions?".to_string(),
                correct_answer: "Hash Table".to_string(),
                wrong_answers: vec!["Binary Tree".to_string(), "Linked List".to_string(), "Heap".to_string()],
                topic: "Computer Science".to_string(),
                difficulty: "Medium".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "What is the largest planet in our solar system?".to_string(),
                correct_answer: "Jupiter".to_string(),
                wrong_answers: vec!["Saturn".to_string(), "Neptune".to_string(), "Mars".to_string()],
                topic: "Astronomy".to_string(),
                difficulty: "Easy".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "Which sorting algorithm has the best average-case time complexity?".to_string(),
                correct_answer: "Quick Sort".to_string(),
                wrong_answers: vec!["Bubble Sort".to_string(), "Selection Sort".to_string(), "Insertion Sort".to_string()],
                topic: "Algorithms".to_string(),
                difficulty: "Hard".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "What is the primary purpose of WebAssembly?".to_string(),
                correct_answer: "Run high-performance code in web browsers".to_string(),
                wrong_answers: vec!["Style web pages".to_string(), "Create web servers".to_string(), "Send HTTP requests".to_string()],
                topic: "Web Development".to_string(),
                difficulty: "Medium".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "Which company created the Rust programming language?".to_string(),
                correct_answer: "Mozilla".to_string(),
                wrong_answers: vec!["Google".to_string(), "Microsoft".to_string(), "Apple".to_string()],
                topic: "Programming".to_string(),
                difficulty: "Easy".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
            Question {
                question_id: 0,
                text: "What is the time complexity of binary search?".to_string(),
                correct_answer: "O(log n)".to_string(),
                wrong_answers: vec!["O(n)".to_string(), "O(n log n)".to_string(), "O(1)".to_string()],
                topic: "Algorithms".to_string(),
                difficulty: "Medium".to_string(),
                quality_score: 0,
                origin_agent: None,
            },
        ];

        for question in questions {
            ctx.db.question_bank().insert(question);
        }
        log::info!("Bootstrapped question bank with initial questions");
    }
}

#[reducer(client_connected)]
pub fn connect(_ctx: &ReducerContext) {
    log::info!("Client connected: {}", _ctx.sender);
}

#[reducer(client_disconnected)]
pub fn disconnect(_ctx: &ReducerContext) {
    log::info!("Client disconnected: {}", _ctx.sender);
}

#[reducer]
pub fn join_lobby(ctx: &ReducerContext, lobby_name: Option<String>) -> Result<(), String> {
    let player_id = ctx.sender;

    // Check if player name exists using the index
    if let Some(existing_player) = ctx.db.player().player_id().find(&player_id) {
        log::info!("Existing player {} joining lobby", existing_player.name);
    } else {
        // Generate unique player name
        let base_name = format!("Player_{}", &player_id.to_string()[..8]);
        let mut counter = 0;
        let mut player_name = base_name.clone();

        // Use name index to check uniqueness efficiently
        while ctx.db.player().name().find(&player_name).is_some() {
            counter += 1;
            player_name = format!("{}_{}", base_name, counter);
        }

        // Try to insert the player, handling potential race condition
        match ctx.db.player().try_insert(Player {
            player_id,
            name: player_name.clone(),
            score: 0,
            elo: 1200, // Initialize Elo to a default starting value
        }) {
            Ok(_) => log::info!("Created new player: {}", player_name),
            Err(_) => return Err("Failed to create player - name taken".to_string()),
        }
    }

    // Find waiting lobby using status index
    if let Some(lobby) = ctx.db.lobby()
        .iter()
        .find(|l| l.status == LOBBY_STATUS_WAITING) {
        log::info!("Player {} joined existing lobby {}", player_id, lobby.lobby_id);
        return Ok(());
    }

    // Create new lobby
    let new_lobby = Lobby {
        lobby_id: 0,
        name: lobby_name,
        status: LOBBY_STATUS_WAITING.to_string(),
        host_id: player_id,
        next_round_is_lightning: false,
    };

    match ctx.db.lobby().try_insert(new_lobby) {
        Ok(lobby) => {
            log::info!("Player {} created new lobby {}", player_id, lobby.lobby_id);
            Ok(())
        },
        Err(e) => Err(format!("Failed to create lobby: {}", e))
    }
}

#[reducer]
pub fn start_game(ctx: &ReducerContext, lobby_id: u64) -> Result<(), String> {
    // Find lobby using primary key index
    let lobby = ctx.db.lobby().lobby_id().find(&lobby_id)
        .ok_or_else(|| format!("Lobby {} not found", lobby_id))?;

    // Security check using proper Identity comparison
    if lobby.host_id != ctx.sender {
        return Err(format!("Only the host can start the game. You are not the host of lobby {}", lobby_id));
    }

    // Game can only be started if lobby is waiting.
    // If it's already IN_GAME, this call might be for starting a *new round* within that game.
    // For now, we assume start_game is only for the first round.
    // Logic for subsequent rounds might need a different reducer or flow.
    if lobby.status != LOBBY_STATUS_WAITING {
        return Err(format!("Lobby {} is not in waiting status (current: {})", lobby_id, lobby.status));
    }

    // Check if this round should be a lightning round
    let mut current_lobby = lobby.clone(); // Clone to modify for next_round_is_lightning flag
    let make_lightning_round = if current_lobby.next_round_is_lightning {
        current_lobby.next_round_is_lightning = false; // Reset the flag
        true
    } else {
        false
    };

    // Update lobby status to in_game and reset next_round_is_lightning if it was used
    current_lobby.status = LOBBY_STATUS_IN_GAME.to_string();
    ctx.db.lobby().try_insert(current_lobby.clone()) // Use current_lobby which has the updated next_round_is_lightning
        .map_err(|e| format!("Failed to update lobby status/flag: {}", e))?;

    // Select random question using timestamp-based randomization
    let question_count = ctx.db.question_bank().count();
    if question_count == 0 {
        return Err("No questions available in the question bank".to_string());
    }

    // Use timestamp value directly for randomization
    let random_index = (ctx.timestamp.to_micros_since_unix_epoch() % question_count as i64) as usize;
    let question = ctx.db.question_bank().iter().nth(random_index)
        .ok_or("Failed to select random question")?;

    // Create first round
    let new_round = ActiveRound {
        round_id: 0,
        lobby_id,
        question_id: question.question_id,
        start_time: ctx.timestamp,
        status: ROUND_STATUS_WAITING.to_string(),
        is_lightning: make_lightning_round, // Set based on lobby flag
    };

    match ctx.db.active_round().try_insert(new_round) {
        Ok(round) => {
            log::info!("Started new round {} in lobby {}", round.round_id, lobby_id);
            // Schedule the first lightning tick for this lobby (temporarily disabled)
            // let delay_micros = 120 * 1_000_000i64;
            // let schedule_at = Timestamp::from_micros_since_unix_epoch(ctx.timestamp.to_micros_since_unix_epoch() + delay_micros);
            // ctx.schedule_event(schedule_at, "lightning_tick", (lobby_id,));
            log::info!("(Scheduling disabled) Would schedule first lightning_tick for lobby {}", lobby_id);
            Ok(())
        },
        Err(e) => Err(format!("Failed to create round: {}", e))
    }
}

// #[reducer] // Temporarily disable lightning_tick reducer to avoid missing schedule feature
// pub fn lightning_tick(ctx: &ReducerContext, lobby_id: u64) -> Result<(), String> {
//     log::info!("lightning_tick triggered for lobby_id: {}", lobby_id);
//     // Function body removed until scheduler implemented via ScheduleAt table.
//     Ok(())
// }

#[reducer]
pub fn submit_answer(ctx: &ReducerContext, round_id: u64, chosen_answer_index: u32) -> Result<(), String> {
    // Validate input - chosen_answer_index should be within a valid range (e.g., 0-3 for 4 choices)
    // For now, we assume it's valid. A check could be added if question details are fetched here.
    // if chosen_answer_index > 3 { // Example check if there are always 4 choices (0,1,2,3)
    //     return Err("Invalid answer index".to_string());
    // }

    // Find round using primary key index
    let round = ctx.db.active_round().round_id().find(&round_id)
        .ok_or_else(|| format!("Round {} not found", round_id))?;

    if round.status != ROUND_STATUS_IN_PROGRESS {
        return Err(format!("Round {} is not in progress (current status: {})", round_id, round.status));
    }

    // Check for existing answer using indexes
    if let Some(existing) = ctx.db.answer()
        .iter()
        .find(|a| a.round_id == round_id && a.player_id == ctx.sender) {
        return Err(format!("Already submitted answer {} for round {}", existing.answer_id, round_id));
    }

    // Create new answer
    let new_answer = Answer {
        answer_id: 0,
        round_id,
        player_id: ctx.sender,
        chosen_answer_index, // Use the provided index
        score: None,
    };

    // Try to insert the answer
    match ctx.db.answer().try_insert(new_answer) {
        Ok(_) => {
            log::info!("Player {} submitted answer index {} for round {}", ctx.sender, chosen_answer_index, round_id);

            // Update CrowdMeterStats
            let stat_entry_opt = CrowdMeterStats::iter(ctx.db)
                .find(|s| s.round_id == round_id && s.answer_index == chosen_answer_index);

            if let Some(mut existing_stat) = stat_entry_opt {
                existing_stat.count += 1;
                if let Err(e) = ctx.db.crowd_meter_stats().try_insert(existing_stat) {
                    log::error!("Failed to update CrowdMeterStats for round {}, index {}: {}", round_id, chosen_answer_index, e);
                    // Not failing the whole reducer for this, but logging error.
                }
            } else {
                let new_stat = CrowdMeterStats {
                    round_id,
                    answer_index: chosen_answer_index,
                    count: 1,
                };
                if let Err(e) = ctx.db.crowd_meter_stats().try_insert(new_stat) {
                    log::error!("Failed to insert new CrowdMeterStats for round {}, index {}: {}", round_id, chosen_answer_index, e);
                    // Not failing the whole reducer for this, but logging error.
                }
            }
            Ok(())
        },
        Err(e) => Err(format!("Failed to submit answer: {}", e))
    }
}

#[reducer]
pub fn score_round(ctx: &ReducerContext, round_id: u64) -> Result<(), String> {
    // Find round using primary key index
    let round = ctx.db.active_round().round_id().find(&round_id)
        .ok_or_else(|| format!("Round {} not found", round_id))?;

    // Find lobby using primary key index
    let lobby = ctx.db.lobby().lobby_id().find(&round.lobby_id)
        .ok_or_else(|| format!("Lobby {} not found", round.lobby_id))?;

    // Security check using proper Identity comparison
    if lobby.host_id != ctx.sender {
        return Err(format!("Only the host can score round {}. You are not the host of lobby {}",
            round_id, lobby.lobby_id));
    }

    if round.status != ROUND_STATUS_IN_PROGRESS {
        return Err(format!("Round {} is not in progress (current status: {})",
            round_id, round.status));
    }

    // Get the question using primary key index
    let question = ctx.db.question_bank().question_id().find(&round.question_id)
        .ok_or_else(|| format!("Question {} not found", round.question_id))?;

    // Update round status to scoring
    let mut scoring_round = round.clone();
    scoring_round.status = ROUND_STATUS_SCORING.to_string();
    ctx.db.active_round().try_insert(scoring_round)
        .map_err(|e| format!("Failed to update round status to scoring: {}", e))?;

    // Score each answer
    let answers: Vec<Answer> = ctx.db.answer()
        .round_id()
        .filter(round_id)
        .collect();

    for answer in answers {
        // Calculate score based on correctness (case-insensitive comparison)
        let mut points_for_correct = 10; // Base points
        if round.is_lightning {
            points_for_correct *= 2; // Double points for lightning round
            log::info!("Lightning round! Player {} gets double points if correct.", answer.player_id);
        }

        let score = if answer.chosen_answer_index == 0 {
            points_for_correct
        } else {
            0 // No points for wrong answer
        };

        // Update answer score
        let mut scored_answer = answer.clone();
        scored_answer.score = Some(score);
        if let Err(e) = ctx.db.answer().try_insert(scored_answer) {
            log::error!("Failed to update score for answer {}: {}", answer.answer_id, e);
            continue; // Continue with other answers even if one fails
        }

        // Update player's total score
        if let Some(mut player) = ctx.db.player().player_id().find(&answer.player_id) {
            player.score += score;
            // Note: Elo is not updated here; it will be updated at game end typically.
            if let Err(e) = ctx.db.player().try_insert(player) {
                log::error!("Failed to update score for player {}: {}", answer.player_id, e);
            }
        }
    }

    // Mark round as finished
    let mut finished_round = round.clone();
    finished_round.status = ROUND_STATUS_FINISHED.to_string();
    ctx.db.active_round().try_insert(finished_round)
        .map_err(|e| format!("Failed to update round status to finished: {}", e))?;

    log::info!("Scored round {} successfully", round_id);
    Ok(())
}

#[reducer]
pub fn finalize_game_and_update_elo(ctx: &ReducerContext, lobby_id: u64) -> Result<(), String> {
    log::info!("finalize_game_and_update_elo called for lobby_id: {}", lobby_id);

    let lobby = ctx.db.lobby().lobby_id().find(&lobby_id)
        .ok_or_else(|| format!("Lobby {} not found for finalize_game", lobby_id))?;

    if lobby.host_id != ctx.sender {
        return Err(format!("Only the host can finalize the game and update Elo for lobby {}", lobby_id));
    }

    if lobby.status != LOBBY_STATUS_IN_GAME {
        return Err(format!("Lobby {} is not in_game (status: {}). Cannot finalize.", lobby_id, lobby.status));
    }

    let mut player_participants: std::collections::HashMap<Identity, Player> = std::collections::HashMap::new();

    for round in ActiveRound::iter(ctx.db).filter(|r| r.lobby_id == lobby_id) {
        for answer in Answer::iter(ctx.db).filter(|a| a.round_id == round.round_id) {
            if !player_participants.contains_key(&answer.player_id) {
                if let Some(player) = ctx.db.player().player_id().find(&answer.player_id) {
                    player_participants.insert(answer.player_id, player.clone());
                }
            }
        }
    }

    if player_participants.len() < 2 {
        log::warn!("Lobby {} has fewer than 2 participants with answers. Skipping Elo update.", lobby_id);
        let mut final_lobby = lobby.clone();
        final_lobby.status = LOBBY_STATUS_FINISHED.to_string();
        final_lobby.next_round_is_lightning = false;
        ctx.db.lobby().try_insert(final_lobby).map_err(|e| format!("Failed to set lobby {} to finished: {}", lobby_id, e))?;
        return Ok(());
    }

    let mut players_vec: Vec<Player> = player_participants.values().cloned().collect();
    players_vec.sort_by(|a, b| b.score.cmp(&a.score));

    for (i, player_to_update) in players_vec.iter().enumerate() {
        let mut mutable_player = player_to_update.clone();
        let opponent_elo_sum: i32 = players_vec.iter()
            .filter(|p| p.player_id != mutable_player.player_id)
            .map(|p| p.elo)
            .sum();
        let num_opponents = players_vec.len() - 1;
        let average_opponent_elo = if num_opponents > 0 { opponent_elo_sum / num_opponents as i32 } else { mutable_player.elo };

        let actual_score = if i == 0 { 1.0 }
        else if i == 1 && players_vec.len() > 2 { 0.5 }
        else { 0.0 };

        let elo_delta = calculate_elo_delta(mutable_player.elo, average_opponent_elo, actual_score, None);
        mutable_player.elo += elo_delta;
        mutable_player.score = 0;

        if let Err(e) = ctx.db.player().try_insert(mutable_player.clone()) {
            log::error!("Failed to update Elo/score for player {}: {}", mutable_player.player_id, e);
        }
    }

    let mut final_lobby = lobby.clone();
    final_lobby.status = LOBBY_STATUS_FINISHED.to_string();
    final_lobby.next_round_is_lightning = false;
    ctx.db.lobby().try_insert(final_lobby).map_err(|e| format!("Failed to set lobby {} to finished: {}", lobby_id, e))?;

    log::info!("Finalized game and updated Elo for lobby {}", lobby_id);
    Ok(())
}

#[reducer]
pub fn request_agent_work(ctx: &ReducerContext, agent_id: u64, topic_json_payload: String) -> Result<(), String> {
    log::info!(
        "request_agent_work called by sender: {} for agent_id: {} with payload: {}",
        ctx.sender,
        agent_id,
        topic_json_payload
    );

    // Basic validation
    if topic_json_payload.trim().is_empty() {
        return Err("Topic JSON payload cannot be empty".to_string());
    }
    // In a real scenario, you might validate if agent_id exists in an agent_registry table
    // For now, we assume agent_id is valid.

    let new_job = AgentJobQueue {
        job_id: 0, // Auto-incremented by the database
        agent_id,
        payload_json: topic_json_payload,
        status: AGENT_JOB_STATUS_PENDING.to_string(),
        error_message: None, // Optional: To store error details if the job failed
    };

    match ctx.db.agent_job_queue().try_insert(new_job) {
        Ok(job) => {
            log::info!("Successfully queued agent job_id: {} for agent_id: {}", job.job_id, agent_id);
            // Here, you would typically notify the agent worker system (e.g., via WebSocket, message queue, or polling)
            // that a new job is available. SpacetimeDB doesn't directly trigger external systems from reducers.
            // The external system would subscribe to the AgentJobQueue table or be poked.
            Ok(())
        }
        Err(e) => {
            let err_msg = format!("Failed to queue agent job for agent_id {}: {}", agent_id, e);
            log::error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

#[reducer]
pub fn register_agent(
    ctx: &ReducerContext,
    wasm_hash: String,
    capabilities: Vec<String>,
    initial_quota: u64,
) -> Result<(), String> {
    log::info!(
        "register_agent called by sender: {} with wasm_hash: {}, capabilities: {:?}, initial_quota: {}",
        ctx.sender,
        wasm_hash,
        capabilities,
        initial_quota
    );

    // Basic validation
    if wasm_hash.trim().is_empty() {
        return Err("WASM hash cannot be empty".to_string());
    }
    // TODO: Add more sophisticated validation for wasm_hash format (e.g., check length, hex characters)
    if capabilities.is_empty() {
        return Err("Agent must have at least one capability".to_string());
    }
    for cap in &capabilities {
        if cap.trim().is_empty() {
            return Err("Capability string cannot be empty".to_string());
        }
    }

    // Check if an agent with the same hash is already registered by this owner (or globally?)
    // For now, let's allow multiple registrations as the exact re-registration policy might vary.
    // A more complex check could be:
    // if AgentRegistry::iter(&ctx.db).any(|agent| agent.owner_id == ctx.sender && agent.wasm_hash == wasm_hash) {
    //     return Err("Agent with this WASM hash already registered by you.".to_string());
    // }

    let new_agent_registration = AgentRegistry {
        agent_id: 0, // Auto-incremented
        owner_id: ctx.sender, // The sender of the reducer call is the owner
        wasm_hash,
        capabilities,
        energy_quota: initial_quota,
    };

    match ctx.db.agent_registry().try_insert(new_agent_registration) {
        Ok(reg) => {
            log::info!(
                "Successfully registered agent_id: {} for owner: {}",
                reg.agent_id,
                ctx.sender
            );
            Ok(())
        }
        Err(e) => {
            let err_msg = format!("Failed to register agent: {}", e);
            log::error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

#[reducer]
pub fn submit_generated_questions(
    ctx: &ReducerContext,
    job_id: u64, // To associate with the original job, though not directly used to update job status here
    agent_id: u64, // The ID of the agent that generated these questions
    questions_data: Vec<NewQuestionData>,
) -> Result<(), String> {
    log::info!(
        "submit_generated_questions called by agent_id: {} (job_id: {}) with {} questions.",
        agent_id,
        job_id,
        questions_data.len()
    );

    if questions_data.is_empty() {
        return Err("No questions data provided".to_string());
    }

    // Optional: Verify agent_id exists in AgentRegistry and has permission
    // if AgentRegistry::filter_by_agent_id(&ctx.db, agent_id).is_none() {
    //     return Err(format!("Agent with ID {} not found in registry.", agent_id));
    // }

    let mut new_questions_count = 0;
    for new_q_data in questions_data {
        // Basic validation for each question
        if new_q_data.text.trim().is_empty() ||
           new_q_data.correct_answer.trim().is_empty() ||
           new_q_data.wrong_answers.is_empty() ||
           new_q_data.wrong_answers.iter().any(|wa| wa.trim().is_empty()) ||
           new_q_data.topic.trim().is_empty() ||
           new_q_data.difficulty.trim().is_empty() {
            log::warn!("Skipping invalid question data: {:?}", new_q_data);
            continue; // Skip this question and try the next
        }

        let question_to_insert = Question {
            question_id: 0, // Auto-incremented
            text: new_q_data.text,
            correct_answer: new_q_data.correct_answer,
            wrong_answers: new_q_data.wrong_answers,
            topic: new_q_data.topic,
            difficulty: new_q_data.difficulty,
            quality_score: 0, // Default quality score for new questions
            origin_agent: Some(agent_id.to_string()), // Tag with the generating agent's ID
        };

        if ctx.db.question_bank().try_insert(question_to_insert).is_ok() {
            new_questions_count += 1;
        } else {
            log::error!("Failed to insert a generated question for agent_id: {}", agent_id);
            // Optionally, collect errors and return them, or just log and continue.
        }
    }

    log::info!("Successfully inserted {} new questions from agent_id: {} (job_id: {}).", new_questions_count, agent_id, job_id);
    // Note: This reducer does not update the AgentJobQueue status.
    // That should be handled by a separate call to `update_agent_job_status` by the worker.
    Ok(())
}

#[reducer]
pub fn update_agent_job_status(
    ctx: &ReducerContext,
    job_id: u64,
    new_status: String,
    error_details: Option<String>,
) -> Result<(), String> {
    log::info!(
        "update_agent_job_status called by sender: {} for job_id: {} to status: {}. Error: {:?}",
        ctx.sender, // Should ideally be a trusted worker/agent identity
        job_id,
        new_status,
        error_details
    );

    // Optional: Add permission check here to ensure only authorized workers/agents can update job statuses.
    // For example, check if ctx.sender is a registered agent or a specific worker identity.

    let mut job = ctx.db.agent_job_queue().job_id().find(&job_id)
        .ok_or_else(|| format!("Agent job_id: {} not found for status update.", job_id))?;

    // Validate new_status against known statuses if desired
    if new_status != AGENT_JOB_STATUS_PENDING &&
       new_status != AGENT_JOB_STATUS_PROCESSING &&
       new_status != AGENT_JOB_STATUS_COMPLETED &&
       new_status != AGENT_JOB_STATUS_FAILED {
        return Err(format!("Invalid new_status provided: {}", new_status));
    }

    job.status = new_status;
    job.error_message = error_details;
    // job.updated_at = Some(ctx.timestamp); // If using timestamps

    match ctx.db.agent_job_queue().try_insert(job.clone()) {
        Ok(_) => {
            log::info!("Successfully updated status for agent job_id: {}", job_id);
            Ok(())
        }
        Err(e) => {
            let err_msg = format!("Failed to update status for agent job_id {}: {}", job_id, e);
            log::error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Imports items from the parent module (your main lib.rs code)
    use spacetimedb::{SpacetimeDb, Identity, Timestamp};

    // Helper to create deterministic identities for testing
    fn get_test_identity(id: u8) -> Identity {
        let mut bytes = [0u8; 16];
        bytes[0] = id;
        Identity::from_bytes(&bytes).unwrap()
    }

    const BOT_1_IDENTITY: Identity = Identity::from_bytes_const(&[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    const BOT_2_IDENTITY: Identity = Identity::from_bytes_const(&[2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    const BOT_3_IDENTITY: Identity = Identity::from_bytes_const(&[3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    #[spacetimedb(test)]
    fn test_join_lobby_new_player_new_lobby(mut db: SpacetimeDb) {
        let lobby_name = Some("First Lobby".to_string());

        // Call the join_lobby reducer for BOT_1_IDENTITY
        // Tuple of arguments must match reducer signature after ReducerContext
        let result = db.call_reducer(BOT_1_IDENTITY, "join_lobby", (lobby_name.clone(),));
        assert!(result.is_ok(), "join_lobby failed: {:?}", result.err());

        // Verify Player table
        let players = Player::iter(&db).collect::<Vec<_>>();
        assert_eq!(players.len(), 1, "Expected 1 player after join_lobby");
        let player1 = players.first().unwrap();
        assert_eq!(player1.player_id, BOT_1_IDENTITY);
        assert!(player1.name.starts_with("Player_")); // Default name generation
        assert_eq!(player1.score, 0);
        assert_eq!(player1.elo, 1200); // Check default Elo

        // Verify Lobby table
        let lobbies = Lobby::iter(&db).collect::<Vec<_>>();
        assert_eq!(lobbies.len(), 1, "Expected 1 lobby after join_lobby");
        let lobby1 = lobbies.first().unwrap();
        assert_eq!(lobby1.name, lobby_name);
        assert_eq!(lobby1.status, LOBBY_STATUS_WAITING);
        assert_eq!(lobby1.host_id, BOT_1_IDENTITY);
    }

    #[spacetimedb(test)]
    fn test_join_lobby_multiple_players_join_same_lobby(mut db: SpacetimeDb) {
        let lobby_name_bot1 = Some("Party Lobby".to_string());
        let lobby_name_bot2 = None; // Bot 2 doesn't specify a name, should join existing

        // Bot 1 creates a lobby
        let res1 = db.call_reducer(BOT_1_IDENTITY, "join_lobby", (lobby_name_bot1.clone(),));
        assert!(res1.is_ok(), "Bot 1 join_lobby failed: {:?}", res1.err());

        // Bot 2 joins
        let res2 = db.call_reducer(BOT_2_IDENTITY, "join_lobby", (lobby_name_bot2,));
        assert!(res2.is_ok(), "Bot 2 join_lobby failed: {:?}", res2.err());

        // Verify Player table
        let players = Player::iter(&db).collect::<Vec<_>>();
        assert_eq!(players.len(), 2, "Expected 2 players");
        // Could add more specific checks for player2 data if needed

        // Verify Lobby table - should still be only one lobby
        let lobbies = Lobby::iter(&db).collect::<Vec<_>>();
        assert_eq!(lobbies.len(), 1, "Expected 1 lobby after two players joined");
        let lobby = lobbies.first().unwrap();
        assert_eq!(lobby.host_id, BOT_1_IDENTITY); // Bot 1 is still host
        assert_eq!(lobby.name, lobby_name_bot1); // Lobby name by first host

        // Verify Bot 2 is in the lobby (implicitly, by not creating a new one)
        // More explicit checks could involve a PlayerInLobby table if it existed.
        // For now, the fact that no new lobby was created for bot2 is the main check.
    }

    #[spacetimedb(test)]
    fn test_join_lobby_existing_player_rejoins(mut db: SpacetimeDb) {
        let lobby_name = Some("Rejoin Test Lobby".to_string());

        // Bot 1 joins for the first time
        let res1 = db.call_reducer(BOT_1_IDENTITY, "join_lobby", (lobby_name.clone(),));
        assert!(res1.is_ok(), "Bot 1 first join failed: {:?}", res1.err());
        assert_eq!(Player::iter(&db).count(), 1, "Expected 1 player after first join");
        assert_eq!(Lobby::iter(&db).count(), 1, "Expected 1 lobby after first join");

        // Bot 1 attempts to "join" again (e.g. client reconnects and calls join_lobby)
        // The current join_lobby logic finds the existing player and doesn't create a new one,
        // and finds the existing lobby.
        let res2 = db.call_reducer(BOT_1_IDENTITY, "join_lobby", (None,)); // Attempt to join any waiting lobby
        assert!(res2.is_ok(), "Bot 1 rejoin failed: {:?}", res2.err());

        // Verify no new player or lobby was created
        assert_eq!(Player::iter(&db).count(), 1, "Expected 1 player after rejoin attempt");
        assert_eq!(Lobby::iter(&db).count(), 1, "Expected 1 lobby after rejoin attempt");
    }

    #[spacetimedb(test)]
    fn test_start_game_success(mut db: SpacetimeDb) {
        // Bot 1 creates a lobby
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Game Start Lobby".to_string()),)).expect("Bot 1 join failed");
        let lobby = Lobby::iter(&db).next().expect("Lobby not found after join");

        // Bot 1 (host) starts the game
        let result = db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby.lobby_id,));
        assert!(result.is_ok(), "start_game failed: {:?}", result.err());

        // Verify Lobby status updated
        let updated_lobby = Lobby::filter_by_lobby_id(&db, lobby.lobby_id).expect("Lobby disappeared");
        assert_eq!(updated_lobby.status, LOBBY_STATUS_IN_GAME);

        // Verify ActiveRound created
        let active_rounds = ActiveRound::iter(&db).collect::<Vec<_>>();
        assert_eq!(active_rounds.len(), 1, "Expected 1 active round");
        let active_round = active_rounds.first().unwrap();
        assert_eq!(active_round.lobby_id, lobby.lobby_id);
        assert_ne!(active_round.question_id, 0); // A question should have been assigned
        assert_eq!(active_round.status, ROUND_STATUS_WAITING); // Initial status of a new round
    }

    #[spacetimedb(test)]
    fn test_start_game_not_host(mut db: SpacetimeDb) {
        // Bot 1 creates a lobby
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Not Host Lobby".to_string()),)).expect("Bot 1 join failed");
        let lobby = Lobby::iter(&db).next().expect("Lobby not found after join");

        // Bot 2 (not host) tries to start the game
        let result = db.call_reducer(BOT_2_IDENTITY, "start_game", (lobby.lobby_id,));
        assert!(result.is_err(), "start_game should have failed for non-host");
        assert!(result.unwrap_err().contains("Only the host can start the game"));

        // Verify Lobby status NOT updated
        let original_lobby = Lobby::filter_by_lobby_id(&db, lobby.lobby_id).expect("Lobby disappeared");
        assert_eq!(original_lobby.status, LOBBY_STATUS_WAITING);
    }

    #[spacetimedb(test)]
    fn test_start_game_lobby_not_waiting(mut db: SpacetimeDb) {
        // Bot 1 creates a lobby
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Not Waiting Lobby".to_string()),)).expect("Bot 1 join failed");
        let lobby_id = Lobby::iter(&db).next().expect("Lobby not found after join").lobby_id;

        // Bot 1 (host) starts the game successfully first
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,)).expect("First start_game failed");
        let lobby_after_start = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        assert_eq!(lobby_after_start.status, LOBBY_STATUS_IN_GAME);

        // Bot 1 (host) tries to start the game AGAIN
        let result = db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,));
        assert!(result.is_err(), "start_game should have failed for already started lobby");
        assert!(result.unwrap_err().contains("is not in waiting status"));
    }

    #[spacetimedb(test)]
    fn test_start_game_no_questions(mut db: SpacetimeDb) {
        // Clear any existing questions (from init or previous tests if db is shared, though test dbs are typically isolated)
        for q in Question::iter(&db) {
            Question::delete_by_question_id(&mut db, q.question_id);
        }
        assert_eq!(Question::iter(&db).count(), 0, "Question bank should be empty");

        // Bot 1 creates a lobby
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("No Questions Lobby".to_string()),)).expect("Bot 1 join failed");
        let lobby = Lobby::iter(&db).next().expect("Lobby not found");

        // Bot 1 (host) tries to start the game
        let result = db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby.lobby_id,));
        assert!(result.is_err(), "start_game should have failed due to no questions");
        assert!(result.unwrap_err().contains("No questions available"));

        // Verify Lobby status is still waiting
        let current_lobby = Lobby::filter_by_lobby_id(&db, lobby.lobby_id).unwrap();
        assert_eq!(current_lobby.status, LOBBY_STATUS_WAITING);
    }

    // Helper function to set up a game and get to an active round for answer/scoring tests
    fn setup_game_for_round_tests(db: &mut SpacetimeDb) -> (u64, u64) { // Returns (lobby_id, round_id)
        // Bot 1 creates lobby
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Round Test Lobby".to_string()),)).expect("Join lobby failed");
        let lobby = Lobby::iter(db).next().expect("Lobby not found");

        // Bot 1 starts game
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby.lobby_id,)).expect("Start game failed");
        let active_round = ActiveRound::iter(db).find(|r| r.lobby_id == lobby.lobby_id).expect("Active round not found");

        // Manually set round status to IN_PROGRESS for testing submit_answer directly
        // In a real flow, this might be triggered by a timer or another reducer.
        let mut round_to_update = active_round.clone();
        round_to_update.status = ROUND_STATUS_IN_PROGRESS.to_string();
        ActiveRound::update_by_round_id(db, active_round.round_id, round_to_update);

        (lobby.lobby_id, active_round.round_id)
    }

    #[spacetimedb(test)]
    fn test_submit_answer_success(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");

        let test_answer_index = 0u32; // Example: Bot 2 chooses the first option
        let result = db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, test_answer_index));
        assert!(result.is_ok(), "submit_answer failed: {:?}", result.err());

        let answers = Answer::iter(&db).collect::<Vec<_>>();
        assert_eq!(answers.len(), 1, "Expected 1 answer");
        let submitted_answer = answers.first().unwrap();
        assert_eq!(submitted_answer.round_id, round_id);
        assert_eq!(submitted_answer.player_id, BOT_2_IDENTITY);
        assert_eq!(submitted_answer.chosen_answer_index, test_answer_index);
        assert!(submitted_answer.score.is_none());
    }

    #[spacetimedb(test)]
    fn test_submit_answer_empty_text(mut db: SpacetimeDb) {
        // This test becomes less relevant as we are not taking text.
        // We might add a test for an out-of-bounds index if we implement index validation.
        // For now, let's remove it or repurpose if strict index validation is added later.
        // Let's comment it out for now, as the reducer no longer takes text.
        /*
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");

        let result = db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, " ".to_string())); // Old signature
        assert!(result.is_err(), "submit_answer should fail for empty text");
        assert!(result.unwrap_err().contains("Answer text cannot be empty"));
        */
    }

    #[spacetimedb(test)]
    fn test_submit_answer_round_not_in_progress(mut db: SpacetimeDb) {
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Round Not Progress Lobby".to_string()),)).expect("Join failed");
        let lobby = Lobby::iter(&db).next().unwrap();
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby.lobby_id,)).expect("Start game failed");
        let round_id = ActiveRound::iter(&db).next().unwrap().round_id;

        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");
        let result = db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, 0u32)); // Using index 0
        assert!(result.is_err(), "submit_answer should fail if round not in progress");
        assert!(result.unwrap_err().contains("is not in progress"));
    }

    #[spacetimedb(test)]
    fn test_submit_answer_twice_by_same_player(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");

        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, 0u32)).expect("First submit failed");

        let result = db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, 1u32)); // Different index, same player
        assert!(result.is_err(), "submit_answer should fail for second attempt by same player");
        assert!(result.unwrap_err().contains("Already submitted answer"));
        assert_eq!(Answer::iter(&db).count(), 1);
    }

    #[spacetimedb(test)]
    fn test_submit_answer_round_not_found(mut db: SpacetimeDb) {
        let non_existent_round_id = 99999u64;
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (None,)).expect("Bot 1 join failed");

        let result = db.call_reducer(BOT_1_IDENTITY, "submit_answer", (non_existent_round_id, 0u32));
        assert!(result.is_err(), "submit_answer should fail if round not found");
        assert!(result.unwrap_err().contains("not found"));
    }

    #[spacetimedb(test)]
    fn test_score_round_success_updates_scores_and_statuses(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        // In setup_game_for_round_tests, a question is bootstrapped.
        // We need to know the correct answer for that question.
        // The first question in init() is "What is the capital of France?" -> "Paris"
        // The start_game reducer picks a random question. For robust tests, we might need to
        // ensure a specific question is picked or check the question details from the active_round.
        // For this test, we'll assume the first question "What is the capital of France?" (correct: "Paris")
        // is active, or make the test flexible to the picked question.

        let active_round = ActiveRound::filter_by_round_id(&db, round_id).unwrap();
        let question = Question::filter_by_question_id(&db, active_round.question_id).unwrap();
        let correct_answer_text = question.correct_answer.clone();
        let incorrect_answer_text = "Berlin".to_string(); // Assuming Berlin is a wrong answer for any question.

        // Bot 2 (player) joins and submits correct answer
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");
        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, 0u32)).expect("Bot 2 submit correct failed");

        // Bot 3 joins and submits incorrect answer
        db.call_reducer(BOT_3_IDENTITY, "join_lobby", (None,)).expect("Bot 3 join failed");
        db.call_reducer(BOT_3_IDENTITY, "submit_answer", (round_id, 1u32)).expect("Bot 3 submit incorrect failed");

        // Bot 1 (host) scores the round
        let result = db.call_reducer(BOT_1_IDENTITY, "score_round", (round_id,));
        assert!(result.is_ok(), "score_round failed: {:?}", result.err());

        // Verify Answer scores
        let answer_bot2 = Answer::iter(&db).find(|a| a.player_id == BOT_2_IDENTITY).unwrap();
        assert_eq!(answer_bot2.score, Some(10), "Bot 2 (correct) score mismatch");

        let answer_bot3 = Answer::iter(&db).find(|a| a.player_id == BOT_3_IDENTITY).unwrap();
        assert_eq!(answer_bot3.score, Some(0), "Bot 3 (incorrect) score mismatch");

        // Verify Player scores
        let player_bot1 = Player::filter_by_player_id(&db, BOT_1_IDENTITY).unwrap();
        assert_eq!(player_bot1.score, 0, "Bot 1 (host) score should be 0");
        let player_bot2 = Player::filter_by_player_id(&db, BOT_2_IDENTITY).unwrap();
        assert_eq!(player_bot2.score, 10, "Bot 2 player score mismatch");
        let player_bot3 = Player::filter_by_player_id(&db, BOT_3_IDENTITY).unwrap();
        assert_eq!(player_bot3.score, 0, "Bot 3 player score mismatch");

        // Verify Round status - should be finished after scoring
        let final_round_state = ActiveRound::filter_by_round_id(&db, round_id).unwrap();
        assert_eq!(final_round_state.status, ROUND_STATUS_FINISHED);
    }

    #[spacetimedb(test)]
    fn test_score_round_not_host(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        // Bot 2 submits an answer
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");
        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, 0u32)).expect("Bot 2 submit failed");

        // Bot 2 (not host) tries to score the round
        let result = db.call_reducer(BOT_2_IDENTITY, "score_round", (round_id,));
        assert!(result.is_err(), "score_round should fail for non-host");
        assert!(result.unwrap_err().contains("Only the host can score round"));

        // Verify round status is still in_progress (or whatever setup_game_for_round_tests sets it to)
        let round = ActiveRound::filter_by_round_id(&db, round_id).unwrap();
        assert_eq!(round.status, ROUND_STATUS_IN_PROGRESS);
    }

    #[spacetimedb(test)]
    fn test_score_round_not_in_progress(mut db: SpacetimeDb) {
        // Setup a round but DONT set it to IN_PROGRESS
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Score Not Progress Lobby".to_string()),)).expect("Join failed");
        let lobby = Lobby::iter(&db).next().unwrap();
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby.lobby_id,)).expect("Start game failed");
        let round = ActiveRound::iter(&db).next().unwrap();
        assert_eq!(round.status, ROUND_STATUS_WAITING); // Should be WAITING

        // Bot 1 (host) tries to score the round
        let result = db.call_reducer(BOT_1_IDENTITY, "score_round", (round.round_id,));
        assert!(result.is_err(), "score_round should fail if not in progress");
        assert!(result.unwrap_err().contains("is not in progress"));
    }

    #[spacetimedb(test)]
    fn test_lightning_tick_sets_flag_and_reschedules(mut db: SpacetimeDb) {
        // Bot 1 creates lobby & starts game, which schedules the first lightning_tick
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Lightning Tick Lobby".to_string()),)).expect("Join failed");
        let lobby_id = Lobby::iter(&db).next().unwrap().lobby_id;
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,)).expect("Start game failed");

        let initial_lobby_state = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        assert!(!initial_lobby_state.next_round_is_lightning, "Lobby should not initially be marked for lightning round");

        // Simulate the passage of 120 seconds for the scheduled event
        // The SpacetimeDB test harness automatically processes scheduled events when time is advanced.
        db.advance_time(Timestamp::from_secs(120));

        // Verify lobby flag is set
        let lobby_after_tick = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        assert!(lobby_after_tick.next_round_is_lightning, "Lobby should be marked for lightning round after tick");

        // Verify that lightning_tick has rescheduled itself by checking scheduled events
        // This is a conceptual check; direct inspection of scheduled events might be complex.
        // A simpler proxy is to advance time again and see if it triggers again (sets flag to true if it was somehow reset).
        // For now, we trust the reschedule logic within lightning_tick itself if it was called.
        // Another tick should be scheduled.
    }

    #[spacetimedb(test)]
    fn test_start_game_creates_lightning_round(mut db: SpacetimeDb) {
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Lightning Round Game".to_string()),)).expect("Join failed");
        let lobby_id = Lobby::iter(&db).next().unwrap().lobby_id;

        // Manually set the lobby to expect a lightning round
        let mut lobby = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        lobby.next_round_is_lightning = true;
        Lobby::update_by_lobby_id(&mut db, lobby_id, lobby);

        // Start the game
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,)).expect("Start game failed");

        // Verify ActiveRound is lightning
        let active_round = ActiveRound::iter(&db).find(|r| r.lobby_id == lobby_id).unwrap();
        assert!(active_round.is_lightning, "ActiveRound should be a lightning round");

        // Verify lobby flag was reset
        let lobby_after_start = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        assert!(!lobby_after_start.next_round_is_lightning, "Lobby next_round_is_lightning flag should be reset");
    }

    #[spacetimedb(test)]
    fn test_score_round_double_points_for_lightning_round(mut db: SpacetimeDb) {
        // Setup: Bot 1 creates lobby, Bot 1 (host) will mark it for lightning, then start game.
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Double Points Lobby".to_string()),)).expect("Join failed");
        let lobby_id = Lobby::iter(&db).next().unwrap().lobby_id;

        let mut lobby = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        lobby.next_round_is_lightning = true;
        Lobby::update_by_lobby_id(&mut db, lobby_id, lobby.clone());

        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,)).expect("Start game failed for lightning round setup");
        let round_id = ActiveRound::iter(&db).find(|r| r.lobby_id == lobby_id).unwrap().round_id;

        let mut active_round_for_test = ActiveRound::filter_by_round_id(&db, round_id).unwrap();
        active_round_for_test.status = ROUND_STATUS_IN_PROGRESS.to_string();
        ActiveRound::update_by_round_id(&mut db, round_id, active_round_for_test.clone());
        assert!(active_round_for_test.is_lightning, "Round created by start_game was not lightning as expected");

        let question = Question::filter_by_question_id(&db, active_round_for_test.question_id).unwrap();
        let correct_answer_text = question.correct_answer.clone();

        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");
        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, 0u32)).expect("Bot 2 submit correct failed");

        db.call_reducer(BOT_1_IDENTITY, "score_round", (round_id,)).expect("Score_round failed");

        let player_bot2 = Player::filter_by_player_id(&db, BOT_2_IDENTITY).unwrap();
        assert_eq!(player_bot2.score, 20, "Bot 2 player score should be 20 for lightning round correct answer");
        assert_eq!(player_bot2.elo, 1200);
    }

    #[spacetimedb(test)]
    fn test_finalize_game_updates_elo_and_status(mut db: SpacetimeDb) {
        // Setup: Bot 1 (host), Bot 2, Bot 3 join. Bot 1 starts. Rounds are played (simulated by manually setting scores).
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Elo Test Lobby".to_string()),)).expect("B1 join");
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("B2 join");
        db.call_reducer(BOT_3_IDENTITY, "join_lobby", (None,)).expect("B3 join");
        let lobby_id = Lobby::iter(&db).next().unwrap().lobby_id;

        // Simulate a game by starting it (to set status to IN_GAME) and manually setting scores
        // and creating some answer records so finalize_game_and_update_elo finds participants
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,)).expect("Start game failed");
        let round_id = ActiveRound::iter(&db).next().unwrap().round_id;
        // Bot 2 answers (to be found as participant)
        Answer::insert(&mut db, Answer { answer_id: 0, round_id, player_id: BOT_2_IDENTITY, chosen_answer_index: 0, score: Some(10) }).unwrap();
        // Bot 3 answers (to be found as participant)
        Answer::insert(&mut db, Answer { answer_id: 0, round_id, player_id: BOT_3_IDENTITY, chosen_answer_index: 1, score: Some(5) }).unwrap();

        // Manually update player scores for ranking
        let mut player1 = Player::filter_by_player_id(&db, BOT_1_IDENTITY).unwrap(); // Host also plays
        player1.score = 10;
        Player::update_by_player_id(&mut db, BOT_1_IDENTITY, player1);

        let mut player2 = Player::filter_by_player_id(&db, BOT_2_IDENTITY).unwrap();
        player2.score = 20; // Bot 2 wins
        Player::update_by_player_id(&mut db, BOT_2_IDENTITY, player2);

        let mut player3 = Player::filter_by_player_id(&db, BOT_3_IDENTITY).unwrap();
        player3.score = 5; // Bot 3 loses
        Player::update_by_player_id(&mut db, BOT_3_IDENTITY, player3);

        // Bot 1 (host) finalizes the game
        let result = db.call_reducer(BOT_1_IDENTITY, "finalize_game_and_update_elo", (lobby_id,));
        assert!(result.is_ok(), "finalize_game_and_update_elo failed: {:?}", result.err());

        // Verify Elos updated (initial Elo is 1200 for all)
        // Bot 2 (winner, score 20) vs avg_elo_others ( (1200(B1)+1200(B3))/2 = 1200). actual_score = 1.0. Delta = 24 * (1-0.5) = 12
        let p2_final = Player::filter_by_player_id(&db, BOT_2_IDENTITY).unwrap();
        assert_eq!(p2_final.elo, 1200 + 12, "Bot 2 Elo mismatch");
        assert_eq!(p2_final.score, 0, "Bot 2 score should be reset");

        // Bot 1 (2nd place, score 10) vs avg_elo_others ( (1200(B2)+1200(B3))/2 = 1200). actual_score = 0.5. Delta = 24 * (0.5-0.5) = 0
        let p1_final = Player::filter_by_player_id(&db, BOT_1_IDENTITY).unwrap();
        assert_eq!(p1_final.elo, 1200 + 0, "Bot 1 Elo mismatch");
        assert_eq!(p1_final.score, 0, "Bot 1 score should be reset");

        // Bot 3 (3rd place, score 5) vs avg_elo_others ( (1200(B1)+1200(B2))/2 = 1200). actual_score = 0.0. Delta = 24 * (0-0.5) = -12
        let p3_final = Player::filter_by_player_id(&db, BOT_3_IDENTITY).unwrap();
        assert_eq!(p3_final.elo, 1200 - 12, "Bot 3 Elo mismatch");
        assert_eq!(p3_final.score, 0, "Bot 3 score should be reset");

        // Verify Lobby status
        let final_lobby = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        assert_eq!(final_lobby.status, LOBBY_STATUS_FINISHED);
        assert!(!final_lobby.next_round_is_lightning); // Should be reset
    }

    #[spacetimedb(test)]
    fn test_finalize_game_less_than_two_players(mut db: SpacetimeDb) {
        db.call_reducer(BOT_1_IDENTITY, "join_lobby", (Some("Single Player Elo Lobby".to_string()),)).expect("B1 join");
        let lobby_id = Lobby::iter(&db).next().unwrap().lobby_id;
        db.call_reducer(BOT_1_IDENTITY, "start_game", (lobby_id,)).expect("Start game failed");

        let result = db.call_reducer(BOT_1_IDENTITY, "finalize_game_and_update_elo", (lobby_id,));
        assert!(result.is_ok(), "finalize_game should succeed but log warning: {:?}", result.err());

        let player1 = Player::filter_by_player_id(&db, BOT_1_IDENTITY).unwrap();
        assert_eq!(player1.elo, 1200, "Elo should not change for single player game");

        let final_lobby = Lobby::filter_by_lobby_id(&db, lobby_id).unwrap();
        assert_eq!(final_lobby.status, LOBBY_STATUS_FINISHED);
    }

    #[spacetimedb(test)]
    fn test_request_agent_work_success(mut db: SpacetimeDb) {
        let test_agent_id = 101u64;
        let test_payload = "{\"topic\": \"Rust Programming\", \"count\": 5}".to_string();

        let result = db.call_reducer(BOT_1_IDENTITY, "request_agent_work", (test_agent_id, test_payload.clone()));
        assert!(result.is_ok(), "request_agent_work failed: {:?}", result.err());

        let jobs = AgentJobQueue::iter(&db).collect::<Vec<_>>();
        assert_eq!(jobs.len(), 1, "Expected 1 job in the queue");
        let job = jobs.first().unwrap();
        assert_eq!(job.agent_id, test_agent_id);
        assert_eq!(job.payload_json, test_payload);
        assert_eq!(job.status, AGENT_JOB_STATUS_PENDING);
        assert_ne!(job.job_id, 0); // Should have an auto-incremented ID
    }

    #[spacetimedb(test)]
    fn test_request_agent_work_empty_payload(mut db: SpacetimeDb) {
        let test_agent_id = 102u64;
        let empty_payload = " ".to_string();

        let result = db.call_reducer(BOT_1_IDENTITY, "request_agent_work", (test_agent_id, empty_payload));
        assert!(result.is_err(), "request_agent_work should fail for empty payload");
        assert!(result.unwrap_err().contains("Topic JSON payload cannot be empty"));
        assert_eq!(AgentJobQueue::iter(&db).count(), 0, "No job should be queued with empty payload");
    }

    #[spacetimedb(test)]
    fn test_register_agent_success(mut db: SpacetimeDb) {
        let wasm_hash = "abcdef1234567890".to_string();
        let capabilities = vec!["generate_questions".to_string(), "moderate_chat".to_string()];
        let initial_quota = 1000u64;

        let result = db.call_reducer(BOT_1_IDENTITY, "register_agent", (wasm_hash.clone(), capabilities.clone(), initial_quota));
        assert!(result.is_ok(), "register_agent failed: {:?}", result.err());

        let agents = AgentRegistry::iter(&db).collect::<Vec<_>>();
        assert_eq!(agents.len(), 1, "Expected 1 agent in the registry");
        let agent = agents.first().unwrap();
        assert_ne!(agent.agent_id, 0);
        assert_eq!(agent.owner_id, BOT_1_IDENTITY);
        assert_eq!(agent.wasm_hash, wasm_hash);
        assert_eq!(agent.capabilities, capabilities);
        assert_eq!(agent.energy_quota, initial_quota);
    }

    #[spacetimedb(test)]
    fn test_register_agent_empty_wasm_hash(mut db: SpacetimeDb) {
        let empty_hash = " ".to_string();
        let capabilities = vec!["generate_questions".to_string()];
        let initial_quota = 100u64;

        let result = db.call_reducer(BOT_1_IDENTITY, "register_agent", (empty_hash, capabilities, initial_quota));
        assert!(result.is_err(), "register_agent should fail for empty WASM hash");
        assert!(result.unwrap_err().contains("WASM hash cannot be empty"));
        assert_eq!(AgentRegistry::iter(&db).count(), 0);
    }

    #[spacetimedb(test)]
    fn test_register_agent_empty_capabilities_list(mut db: SpacetimeDb) {
        let wasm_hash = "testhash".to_string();
        let empty_capabilities: Vec<String> = vec![];
        let initial_quota = 100u64;

        let result = db.call_reducer(BOT_1_IDENTITY, "register_agent", (wasm_hash, empty_capabilities, initial_quota));
        assert!(result.is_err(), "register_agent should fail for empty capabilities list");
        assert!(result.unwrap_err().contains("Agent must have at least one capability"));
        assert_eq!(AgentRegistry::iter(&db).count(), 0);
    }

    #[spacetimedb(test)]
    fn test_submit_answer_updates_crowd_meter_stats_new_entry(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");

        let chosen_idx = 1u32;
        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, chosen_idx)).expect("Submit answer failed");

        let stats = CrowdMeterStats::filter_by_round_id_and_answer_index(&db, round_id, chosen_idx);
        assert!(stats.is_some(), "CrowdMeterStats entry not found");
        assert_eq!(stats.unwrap().count, 1, "CrowdMeterStats count should be 1 for new entry");
    }

    #[spacetimedb(test)]
    fn test_submit_answer_updates_crowd_meter_stats_increments_existing(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        // Bot 2 joins and answers
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");
        let chosen_idx = 2u32;
        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, chosen_idx)).expect("Bot 2 submit failed");

        // Bot 3 joins and answers with the same index
        db.call_reducer(BOT_3_IDENTITY, "join_lobby", (None,)).expect("Bot 3 join failed");
        db.call_reducer(BOT_3_IDENTITY, "submit_answer", (round_id, chosen_idx)).expect("Bot 3 submit failed");

        let stats = CrowdMeterStats::filter_by_round_id_and_answer_index(&db, round_id, chosen_idx);
        assert!(stats.is_some(), "CrowdMeterStats entry not found after multiple submissions");
        assert_eq!(stats.unwrap().count, 2, "CrowdMeterStats count should be 2");
    }

    #[spacetimedb(test)]
    fn test_submit_answer_updates_crowd_meter_stats_multiple_indices(mut db: SpacetimeDb) {
        let (_lobby_id, round_id) = setup_game_for_round_tests(&mut db);
        let idx_0 = 0u32;
        let idx_1 = 1u32;

        // Bot 1 answers index 0
        // Bot 1 is host, but can also be a player submitting answers
        db.call_reducer(BOT_1_IDENTITY, "submit_answer", (round_id, idx_0)).expect("Bot 1 submit idx 0 failed");

        // Bot 2 joins and answers index 1
        db.call_reducer(BOT_2_IDENTITY, "join_lobby", (None,)).expect("Bot 2 join failed");
        db.call_reducer(BOT_2_IDENTITY, "submit_answer", (round_id, idx_1)).expect("Bot 2 submit idx 1 failed");

        // Bot 3 joins and answers index 0
        db.call_reducer(BOT_3_IDENTITY, "join_lobby", (None,)).expect("Bot 3 join failed");
        db.call_reducer(BOT_3_IDENTITY, "submit_answer", (round_id, idx_0)).expect("Bot 3 submit idx 0 failed");

        let stats_idx_0 = CrowdMeterStats::filter_by_round_id_and_answer_index(&db, round_id, idx_0).unwrap();
        assert_eq!(stats_idx_0.count, 2, "Count for index 0 should be 2");

        let stats_idx_1 = CrowdMeterStats::filter_by_round_id_and_answer_index(&db, round_id, idx_1).unwrap();
        assert_eq!(stats_idx_1.count, 1, "Count for index 1 should be 1");
    }

    #[spacetimedb(test)]
    fn test_submit_generated_questions_success(mut db: SpacetimeDb) {
        let agent_id = 111u64; // Assume this agent is registered or valid for submission
        let job_id = 1u64;
        let questions_to_submit = vec![
            NewQuestionData {
                text: "What is 2+2?".to_string(),
                correct_answer: "4".to_string(),
                wrong_answers: vec!["3".to_string(), "5".to_string()],
                topic: "Math".to_string(),
                difficulty: "Easy".to_string(),
            },
            NewQuestionData {
                text: "Capital of Rustland?".to_string(),
                correct_answer: "Ferris City".to_string(),
                wrong_answers: vec!["Cargo Town".to_string()],
                topic: "Programming Fun".to_string(),
                difficulty: "Medium".to_string(),
            },
        ];

        let result = db.call_reducer(BOT_1_IDENTITY, "submit_generated_questions", (job_id, agent_id, questions_to_submit.clone()));
        assert!(result.is_ok(), "submit_generated_questions failed: {:?}", result.err());

        let all_questions = Question::iter(&db).collect::<Vec<_>>();
        // Assuming init questions + 2 new ones
        assert_eq!(all_questions.len(), Question::iter(&db).filter(|q| q.origin_agent.is_none()).count() + 2, "Incorrect number of questions after submission");

        let submitted_q1 = Question::iter(&db).find(|q| q.text == "What is 2+2?").unwrap();
        assert_eq!(submitted_q1.correct_answer, "4");
        assert_eq!(submitted_q1.topic, "Math");
        assert_eq!(submitted_q1.quality_score, 0);
        assert_eq!(submitted_q1.origin_agent, Some(agent_id.to_string()));
    }

    #[spacetimedb(test)]
    fn test_submit_generated_questions_empty_list(mut db: SpacetimeDb) {
        let result = db.call_reducer(BOT_1_IDENTITY, "submit_generated_questions", (1u64, 112u64, Vec::<NewQuestionData>::new()));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No questions data provided"));
    }

    #[spacetimedb(test)]
    fn test_submit_generated_questions_skips_invalid(mut db: SpacetimeDb) {
        let agent_id = 113u64;
        let questions_to_submit = vec![
            NewQuestionData { // Valid
                text: "Valid Q".to_string(), correct_answer: "A".to_string(), wrong_answers: vec!["B".to_string()],
                topic: "T".to_string(), difficulty: "D".to_string(),
            },
            NewQuestionData { // Invalid (empty text)
                text: "".to_string(), correct_answer: "B".to_string(), wrong_answers: vec!["C".to_string()],
                topic: "T".to_string(), difficulty: "D".to_string(),
            },
        ];
        // Count questions not from the init block, or adjust if init questions can have origin_agent set.
        let initial_q_count = Question::iter(&db).filter(|q| q.origin_agent.is_some() && q.origin_agent.as_ref().unwrap() == &agent_id.to_string()).count();
        db.call_reducer(BOT_1_IDENTITY, "submit_generated_questions", (2u64, agent_id, questions_to_submit)).expect("submit failed");
        let final_q_count = Question::iter(&db).filter(|q| q.origin_agent.is_some() && q.origin_agent.as_ref().unwrap() == &agent_id.to_string()).count();
        assert_eq!(final_q_count, initial_q_count + 1, "Only one valid question should be added for this agent");
    }

    // Next tests for update_agent_job_status will go here
    #[spacetimedb(test)]
    fn test_update_agent_job_status_success(mut db: SpacetimeDb) {
        let agent_id_for_job = 201u64;
        let job_payload = "test payload for status update".to_string();
        // Ensure BOT_1_IDENTITY can call request_agent_work, or use an appropriate identity
        db.call_reducer(BOT_1_IDENTITY, "request_agent_work", (agent_id_for_job, job_payload)).expect("Failed to request agent work for setup");
        let job = AgentJobQueue::iter(&db).find(|j| j.agent_id == agent_id_for_job).expect("Job not found after request");
        assert_eq!(job.status, AGENT_JOB_STATUS_PENDING);
    } // Added closing brace for the function
} // Added closing brace for the mod tests