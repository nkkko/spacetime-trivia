"use client"

import { useState } from "react"
import { LandingScreen } from "@/components/screens/landing-screen"
import { CreateLobbyModal } from "@/components/modals/create-lobby-modal"
import { WaitingRoomScreen } from "@/components/screens/waiting-room-screen"
import { QuestionRoundScreen } from "@/components/screens/question-round-screen"
import { RoundResultScreen } from "@/components/screens/round-result-screen"
import { LightningRoundScreen } from "@/components/screens/lightning-round-screen"
import { GameOverScreen } from "@/components/screens/game-over-screen"
import { AgentStudioScreen } from "@/components/screens/agent-studio-screen"

export default function Home() {
  // For demo purposes, we'll use state to switch between screens
  const [currentScreen, setCurrentScreen] = useState("landing")
  const [showCreateLobby, setShowCreateLobby] = useState(false)

  // Mock data for the topic cards
  const topics = [
    { id: 1, title: "90s Pop", color: "#FF2D9B", playerCount: 1243 },
    { id: 2, title: "Science", color: "#4CAF50", playerCount: 876 },
    { id: 3, title: "Movies", color: "#FF9800", playerCount: 2134 },
    { id: 4, title: "History", color: "#2196F3", playerCount: 543 },
    { id: 5, title: "Sports", color: "#F44336", playerCount: 987 },
    { id: 6, title: "Tech", color: "#9C27B0", playerCount: 1567 },
  ]

  const handleCreateLobby = () => {
    setShowCreateLobby(true)
  }

  const handleLobbyCreated = () => {
    setShowCreateLobby(false)
    setCurrentScreen("waitingRoom")
  }

  const handleStartGame = () => {
    setCurrentScreen("questionRound")
  }

  const handleNextRound = () => {
    setCurrentScreen("roundResult")
  }

  const handleLightningRound = () => {
    setCurrentScreen("lightningRound")
  }

  const handleGameOver = () => {
    setCurrentScreen("gameOver")
  }

  const handlePlayAgain = () => {
    setCurrentScreen("landing")
  }

  const handleAgentStudio = () => {
    setCurrentScreen("agentStudio")
  }

  return (
    <main className="min-h-screen bg-slate-950 text-white">
      {currentScreen === "landing" && <LandingScreen topics={topics} onCreateLobby={handleCreateLobby} />}

      {currentScreen === "waitingRoom" && <WaitingRoomScreen onStartGame={handleStartGame} />}

      {currentScreen === "questionRound" && <QuestionRoundScreen onNextRound={handleNextRound} />}

      {currentScreen === "roundResult" && <RoundResultScreen onNextRound={handleLightningRound} isCorrect={true} />}

      {currentScreen === "lightningRound" && <LightningRoundScreen onNextRound={handleGameOver} />}

      {currentScreen === "gameOver" && (
        <GameOverScreen onPlayAgain={handlePlayAgain} onAgentStudio={handleAgentStudio} />
      )}

      {currentScreen === "agentStudio" && <AgentStudioScreen onBack={() => setCurrentScreen("landing")} />}

      {showCreateLobby && (
        <CreateLobbyModal onClose={() => setShowCreateLobby(false)} onCreateLobby={handleLobbyCreated} />
      )}
    </main>
  )
}
