"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"
import { AnswerButton } from "@/components/ui-components/answer-button"
import { TimerRing } from "@/components/ui-components/timer-ring"
import { CrowdMeter } from "@/components/ui-components/crowd-meter"
import { EmojiBurst } from "@/components/ui-components/emoji-burst"
import { AvatarStack } from "@/components/ui-components/avatar-stack"
import { Flag, ThumbsUp, ThumbsDown } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface QuestionRoundScreenProps {
  onNextRound: () => void
}

export function QuestionRoundScreen({ onNextRound }: QuestionRoundScreenProps) {
  const isMobile = useMobile()
  const [selectedAnswer, setSelectedAnswer] = useState<number | null>(null)
  const [showEmojiBurst, setShowEmojiBurst] = useState(false)
  const [crowdAnswers, setCrowdAnswers] = useState([0, 0, 0, 0])
  const [firstCorrectPlayers, setFirstCorrectPlayers] = useState<{ id: string; name: string }[]>([])

  const accentColor = "#FF2D9B" // 90s Pop color

  // Mock question data
  const question = {
    text: "Which 90s pop group released the hit song 'Wannabe' in 1996?",
    answers: ["Backstreet Boys", "Spice Girls", "NSYNC", "Destiny's Child"],
    correctIndex: 1,
  }

  // Simulate crowd answers
  useEffect(() => {
    const interval = setInterval(() => {
      setCrowdAnswers((prev) => {
        // Bias towards correct answer
        const newAnswers = [...prev]
        const randomIndex = Math.random() < 0.6 ? question.correctIndex : Math.floor(Math.random() * 4)
        newAnswers[randomIndex] += Math.floor(Math.random() * 3) + 1
        return newAnswers
      })
    }, 300)

    return () => clearInterval(interval)
  }, [question.correctIndex])

  // Simulate first correct players
  useEffect(() => {
    const names = ["Alex", "Taylor", "Jordan", "Casey", "Riley"]

    setTimeout(() => {
      const players = names.map((name, index) => ({
        id: `player-${index}`,
        name: `${name}${Math.floor(Math.random() * 100)}`,
      }))

      setFirstCorrectPlayers(players)
    }, 2000)
  }, [])

  const handleSelectAnswer = (index: number) => {
    if (selectedAnswer !== null) return

    setSelectedAnswer(index)

    if (index === question.correctIndex) {
      setShowEmojiBurst(true)
      setTimeout(() => setShowEmojiBurst(false), 1000)
    }
  }

  return (
    <div className="min-h-screen p-4 md:p-8 relative">
      {showEmojiBurst && <EmojiBurst emoji="🎉" />}

      <div className="max-w-6xl mx-auto">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <motion.div
            className="glass-card p-6 lg:col-span-2"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3 }}
          >
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-xl font-heading font-bold">Question 1/10</h2>
              <TimerRing duration={15} accentColor={accentColor} onComplete={onNextRound} />
            </div>

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
                  isSelected={selectedAnswer === index}
                  onClick={() => handleSelectAnswer(index)}
                />
              ))}
            </div>

            <div className="flex justify-center gap-4">
              <button className="p-2 rounded-full hover:bg-slate-800/50">
                <ThumbsUp size={20} className="text-slate-400 hover:text-green-500" />
              </button>
              <button className="p-2 rounded-full hover:bg-slate-800/50">
                <ThumbsDown size={20} className="text-slate-400 hover:text-red-500" />
              </button>
              <button className="p-2 rounded-full hover:bg-slate-800/50">
                <Flag size={20} className="text-slate-400 hover:text-yellow-500" />
              </button>
            </div>
          </motion.div>

          <div className="space-y-6">
            <motion.div
              className="glass-card p-6"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: 0.1 }}
            >
              <h2 className="text-xl font-heading font-bold mb-4">Crowd Meter</h2>
              <CrowdMeter answers={crowdAnswers} accentColor={accentColor} />
            </motion.div>

            <motion.div
              className="glass-card p-6"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: 0.2 }}
            >
              <h2 className="text-xl font-heading font-bold mb-4">First 5 Correct</h2>
              <AvatarStack
                avatars={firstCorrectPlayers.map((p) => ({ id: p.id, name: p.name }))}
                showAnimation={true}
                accentColor={accentColor}
              />
            </motion.div>
          </div>
        </div>
      </div>
    </div>
  )
}
