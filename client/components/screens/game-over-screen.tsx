"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"
import { EmojiBurst } from "@/components/ui-components/emoji-burst"
import { Trophy, Medal, Share2, RotateCcw, Sparkles } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface GameOverScreenProps {
  onPlayAgain: () => void
  onAgentStudio: () => void
}

export function GameOverScreen({ onPlayAgain, onAgentStudio }: GameOverScreenProps) {
  const isMobile = useMobile()
  const [showEmojiBurst, setShowEmojiBurst] = useState(true)

  // Mock player stats
  const playerStats = {
    score: 1250,
    rank: 42,
    totalPlayers: 1243,
    correctAnswers: 8,
    totalQuestions: 10,
    streak: 5,
    badges: [
      { id: 1, name: "Speed Demon", description: "Answered 3 questions in under 2 seconds" },
      { id: 2, name: "Combo Master", description: "5 correct answers in a row" },
      { id: 3, name: "90s Expert", description: "Scored in the top 10% for 90s Pop" },
    ],
  }

  // Hide emoji burst after a delay
  useEffect(() => {
    const timer = setTimeout(() => {
      setShowEmojiBurst(false)
    }, 3000)

    return () => clearTimeout(timer)
  }, [])

  return (
    <div className="min-h-screen p-4 md:p-8 relative">
      {showEmojiBurst && <EmojiBurst emoji="🏆" count={20} duration={3000} />}

      <div className="max-w-4xl mx-auto">
        <motion.div
          className="glass-card p-8 text-center mb-8"
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5, type: "spring" }}
        >
          <motion.h1
            className="text-4xl md:text-5xl font-heading font-bold mb-2 bg-gradient-to-r from-yellow-400 to-pink-500 text-transparent bg-clip-text"
            initial={{ y: -50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.2 }}
          >
            Game Over!
          </motion.h1>

          <motion.p
            className="text-xl text-slate-300 mb-8"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.3 }}
          >
            You scored {playerStats.score} points
          </motion.p>

          <motion.div
            className="flex flex-col md:flex-row justify-center gap-6 mb-8"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.4 }}
          >
            <div className="glass-card p-6 flex-1">
              <div className="w-12 h-12 rounded-full bg-yellow-500/20 flex items-center justify-center mx-auto mb-4">
                <Trophy size={24} className="text-yellow-500" />
              </div>
              <div className="text-2xl font-bold mb-1">#{playerStats.rank}</div>
              <div className="text-sm text-slate-400">Out of {playerStats.totalPlayers} players</div>
            </div>

            <div className="glass-card p-6 flex-1">
              <div className="w-12 h-12 rounded-full bg-green-500/20 flex items-center justify-center mx-auto mb-4">
                <Medal size={24} className="text-green-500" />
              </div>
              <div className="text-2xl font-bold mb-1">
                {playerStats.correctAnswers}/{playerStats.totalQuestions}
              </div>
              <div className="text-sm text-slate-400">Correct answers</div>
            </div>

            <div className="glass-card p-6 flex-1">
              <div className="w-12 h-12 rounded-full bg-purple-500/20 flex items-center justify-center mx-auto mb-4">
                <Share2 size={24} className="text-purple-500" />
              </div>
              <div className="text-2xl font-bold mb-1">{playerStats.streak}x</div>
              <div className="text-sm text-slate-400">Best streak</div>
            </div>
          </motion.div>

          <motion.div
            className="mb-8"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.5 }}
          >
            <h2 className="text-xl font-heading font-bold mb-4">Badges Earned</h2>

            <div className="flex flex-wrap justify-center gap-4">
              {playerStats.badges.map((badge) => (
                <div
                  key={badge.id}
                  className="glass-card p-4 w-full md:w-auto"
                  style={{
                    background: "linear-gradient(135deg, rgba(236, 72, 153, 0.2), rgba(124, 58, 237, 0.2))",
                    borderColor: "rgba(236, 72, 153, 0.3)",
                  }}
                >
                  <div className="text-lg font-bold mb-1">{badge.name}</div>
                  <div className="text-sm text-slate-300">{badge.description}</div>
                </div>
              ))}
            </div>
          </motion.div>

          <motion.div
            className="flex flex-col md:flex-row justify-center gap-4"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.6 }}
          >
            <button
              className="px-6 py-3 rounded-lg bg-slate-800 text-white flex items-center justify-center gap-2 hover:bg-slate-700"
              onClick={onPlayAgain}
            >
              <RotateCcw size={18} />
              Play Again
            </button>

            <button
              className="px-6 py-3 rounded-lg bg-gradient-to-r from-pink-500 to-purple-600 text-white flex items-center justify-center gap-2"
              onClick={onAgentStudio}
            >
              <Sparkles size={18} />
              Create Your Own Quiz
            </button>
          </motion.div>
        </motion.div>

        <motion.div
          className="glass-card p-6"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.7 }}
        >
          <h2 className="text-xl font-heading font-bold mb-4">Global Leaderboard</h2>

          <div className="space-y-2">
            {Array.from({ length: 5 }).map((_, index) => (
              <div
                key={index}
                className={`flex items-center justify-between p-3 rounded-lg ${index === 1 ? "bg-pink-500/20 border border-pink-500/30" : "hover:bg-slate-800/50"}`}
              >
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-full bg-slate-800 flex items-center justify-center text-sm font-bold">
                    {index + 1}
                  </div>
                  <span>{index === 1 ? "You" : `Player${Math.floor(Math.random() * 1000)}`}</span>
                </div>
                <div className="font-bold">
                  {index === 1 ? playerStats.score : Math.floor(1000 + Math.random() * 1000)}
                </div>
              </div>
            ))}
          </div>
        </motion.div>
      </div>
    </div>
  )
}
