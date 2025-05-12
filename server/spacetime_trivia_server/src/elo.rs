const DEFAULT_K_FACTOR: f32 = 24.0;

/// Calculates the expected score for a player based on their rating and the opponent's rating.
/// Expected score is the probability of the player winning against the opponent.
/// P(A) = 1 / (1 + 10^((RatingB - RatingA) / 400))
fn calculate_expected_score(player_rating: i32, opponent_rating: i32) -> f32 {
    1.0 / (1.0 + 10.0f32.powf((opponent_rating - player_rating) as f32 / 400.0))
}

/// Calculates the change in Elo rating for a player.
///
/// # Arguments
/// * `player_current_elo` - The current Elo rating of the player.
/// * `opponent_elo` - The Elo rating of the opponent (or average rating, or item difficulty).
/// * `actual_score` - The player's actual score against the opponent (1.0 for win, 0.5 for draw, 0.0 for loss).
/// * `k_factor` - The K-factor, determining the sensitivity of rating changes. Defaults to `DEFAULT_K_FACTOR` if None.
///
/// # Returns
/// The change (delta) in Elo rating. Add this to the player's current Elo.
pub fn calculate_elo_delta(player_current_elo: i32, opponent_elo: i32, actual_score: f32, k_factor: Option<f32>) -> i32 {
    let k = k_factor.unwrap_or(DEFAULT_K_FACTOR);
    let expected_score = calculate_expected_score(player_current_elo, opponent_elo);
    let elo_change = k * (actual_score - expected_score);
    elo_change.round() as i32
}


#[cfg(test)]
mod tests {
    use super::*; // Imports items from the parent module (elo.rs)

    #[test]
    fn test_calculate_expected_score_equal_ratings() {
        assert_eq!(calculate_expected_score(1200, 1200), 0.5);
    }

    #[test]
    fn test_calculate_expected_score_player_stronger() {
        // Player is 200 points stronger, should have > 0.5 chance
        let expected = calculate_expected_score(1400, 1200);
        assert!(expected > 0.5 && expected < 1.0);
        // Expected value for 1400 vs 1200 is ~0.76
        assert!((expected - 0.7597).abs() < 0.001);
    }

    #[test]
    fn test_calculate_expected_score_player_weaker() {
        // Player is 200 points weaker, should have < 0.5 chance
        let expected = calculate_expected_score(1200, 1400);
        assert!(expected < 0.5 && expected > 0.0);
        // Expected value for 1200 vs 1400 is ~0.24
        assert!((expected - 0.2402).abs() < 0.001);
    }

    #[test]
    fn test_calculate_elo_delta_win_equal_ratings() {
        // Player wins against an equally rated opponent
        let delta = calculate_elo_delta(1200, 1200, 1.0, Some(24.0));
        assert_eq!(delta, 12); // K/2
    }

    #[test]
    fn test_calculate_elo_delta_loss_equal_ratings() {
        // Player loses against an equally rated opponent
        let delta = calculate_elo_delta(1200, 1200, 0.0, Some(24.0));
        assert_eq!(delta, -12); // -K/2
    }

    #[test]
    fn test_calculate_elo_delta_draw_equal_ratings() {
        // Player draws against an equally rated opponent
        let delta = calculate_elo_delta(1200, 1200, 0.5, Some(24.0));
        assert_eq!(delta, 0);
    }

    #[test]
    fn test_calculate_elo_delta_strong_player_wins_against_weak() {
        // Stronger player (1400) wins against weaker (1200), less ELO gain
        let delta = calculate_elo_delta(1400, 1200, 1.0, Some(24.0));
        // Expected score for 1400 vs 1200 is ~0.76
        // Change = 24 * (1.0 - 0.76) = 24 * 0.24 = 5.76 -> rounded to 6
        assert_eq!(delta, 6);
    }

    #[test]
    fn test_calculate_elo_delta_weak_player_wins_against_strong() {
        // Weaker player (1200) wins against stronger (1400), more ELO gain
        let delta = calculate_elo_delta(1200, 1400, 1.0, Some(24.0));
        // Expected score for 1200 vs 1400 is ~0.24
        // Change = 24 * (1.0 - 0.24) = 24 * 0.76 = 18.24 -> rounded to 18
        assert_eq!(delta, 18);
    }

    #[test]
    fn test_calculate_elo_delta_uses_default_k_factor() {
        // Test that if k_factor is None, DEFAULT_K_FACTOR (24.0) is used
        let delta_custom_k = calculate_elo_delta(1200, 1200, 1.0, Some(32.0));
        assert_eq!(delta_custom_k, 16); // 32 * (1.0 - 0.5)

        let delta_default_k = calculate_elo_delta(1200, 1200, 1.0, None);
        assert_eq!(delta_default_k, 12); // Should use DEFAULT_K_FACTOR (24.0 * 0.5)
    }
}