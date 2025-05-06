"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"
import { AnswerButton } from "@/components/ui-components/answer-button"
import { EmojiBurst } from "@/components/ui-components/emoji-burst"
import { Trophy, Zap, Clock } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface RoundResultScreenProps {
  onNextRound: () => void
  isCorrect: boolean
}

export function RoundResultScreen({ onNextRound, isCorrect }: RoundResultScreenProps) {
  const isMobile = useMobile()
  const [showEmojiBurst, setShowEmojiBurst] = useState(isCorrect)
  const [timeToNextRound, setTimeToNextRound] = useState(5)

  const accentColor = "#FF2D9B" // 90s Pop color

  // Mock question data
  const question = {
    text: "Which 90s pop group released the hit song 'Wannabe' in 1996?",
    answers: ["Backstreet Boys", "Spice Girls", "NSYNC", "Destiny's Child"],
    correctIndex: 1,
    selectedIndex: 1, // For demo, assuming user selected correctly
  }

  // Countdown to next round
  useEffect(() => {
    const interval = setInterval(() => {
      setTimeToNextRound((prev) => {
        if (prev <= 1) {
          clearInterval(interval)
          setTimeout(onNextRound, 1000)
          return 0
        }
        return prev - 1
      })
    }, 1000)

    return () => clearInterval(interval)
  }, [onNextRound])

  // Hide emoji burst after a delay
  useEffect(() => {
    if (showEmojiBurst) {
      const timer = setTimeout(() => {
        setShowEmojiBurst(false)
      }, 2000)

      return () => clearTimeout(timer)
    }
  }, [showEmojiBurst])

  // Mock stats
  const stats = {
    streak: 3,
    responseTime: 1.2, // seconds
    rank: 42,
    totalPlayers: 1243,
  }

  return (
    <div className="min-h-screen p-4 md:p-8 relative">
      {showEmojiBurst && <EmojiBurst emoji={isCorrect ? "🎉" : "😢"} />}

      <div className="max-w-6xl mx-auto">
        <motion.div
          className="glass-card p-6 mb-6"
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.3 }}
        >
          <div className="flex justify-between items-center">
            <h2 className="text-xl font-heading font-bold">{isCorrect ? "Correct!" : "Incorrect!"}</h2>
            <div className="text-slate-400">Next round in {timeToNextRound}s</div>
          </div>
        </motion.div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <motion.div
            className="glass-card p-6 lg:col-span-2"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3 }}
          >
            <motion.h1
              className="text-2xl md:text-3xl font-heading font-bold mb-8"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.5, delay: 0.2 }}
            >
              {question.text}
            </motion.h1>

            <div className="space-y-4 mb-8">
              {question.answers.map((answer, index) => (
                <AnswerButton
                  key={index}
                  text={answer}
                  index={index}
                  accentColor={accentColor}
                  isSelected={question.selectedIndex === index}
                  isCorrect={question.correctIndex === index ? true : null}
                  isWrong={question.selectedIndex === index && question.correctIndex !== index ? true : null}
                  isDisabled={true}
                  onClick={() => {}}
                />
              ))}
            </div>
          </motion.div>

          <div className="space-y-6">
            <motion.div
              className="glass-card p-6"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: 0.1 }}
            >
              <h2 className="text-xl font-heading font-bold mb-4">Your Stats</h2>

              <div className="space-y-4">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-yellow-500/20 flex items-center justify-center">
                    <Trophy size={20} className="text-yellow-500" />
                  </div>
                  <div>
                    <div className="text-sm text-slate-400">Current Streak</div>
                    <div className="text-xl font-bold">{stats.streak}x</div>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center">
                    <Clock size={20} className="text-blue-500" />
                  </div>
                  <div>
                    <div className="text-sm text-slate-400">Response Time</div>
                    <div className="text-xl font-bold">{stats.responseTime}s</div>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-purple-500/20 flex items-center justify-center">
                    <Zap size={20} className="text-purple-500" />
                  </div>
                  <div>
                    <div className="text-sm text-slate-400">Rank</div>
                    <div className="text-xl font-bold">
                      {stats.rank}/{stats.totalPlayers}
                    </div>
                  </div>
                </div>
              </div>
            </motion.div>
          </div>
        </div>
      </div>
    </div>
  )
}
