"use client"

import { useState } from "react"
import { motion } from "framer-motion"
import { AnswerButton } from "@/components/ui-components/answer-button"
import { TimerRing } from "@/components/ui-components/timer-ring"
import { CrowdMeter } from "@/components/ui-components/crowd-meter"
import { LightningBanner } from "@/components/ui-components/lightning-banner"
import { EmojiBurst } from "@/components/ui-components/emoji-burst"
import { Flag, ThumbsUp, ThumbsDown } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface LightningRoundScreenProps {
  onNextRound: () => void
}

export function LightningRoundScreen({ onNextRound }: LightningRoundScreenProps) {
  const isMobile = useMobile()
  const [selectedAnswer, setSelectedAnswer] = useState<number | null>(null)
  const [showEmojiBurst, setShowEmojiBurst] = useState(false)

  const accentColor = "#FF2D9B" // 90s Pop color

  // Mock question data
  const question = {
    text: "Which of these songs was NOT a hit for Britney Spears in the 90s?",
    answers: ["...Baby One More Time", "Oops!...I Did It Again", "Toxic", "Sometimes"],
    correctIndex: 2,
  }

  // Mock crowd answers
  const crowdAnswers = [15, 12, 25, 8]

  const handleSelectAnswer = (index: number) => {
    if (selectedAnswer !== null) return

    setSelectedAnswer(index)

    if (index === question.correctIndex) {
      setShowEmojiBurst(true)
      setTimeout(() => setShowEmojiBurst(false), 1000)
    }

    // Move to next round after a delay
    setTimeout(onNextRound, 2000)
  }

  return (
    <div className="min-h-screen p-4 md:p-8 relative">
      <LightningBanner visible={true} />

      {showEmojiBurst && <EmojiBurst emoji="⚡" />}

      <div className="max-w-6xl mx-auto mt-16">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <motion.div
            className="glass-card p-6 lg:col-span-2 border-2 border-red-500/30"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3 }}
            style={{ boxShadow: "0 0 20px rgba(239, 68, 68, 0.2)" }}
          >
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-xl font-heading font-bold text-red-500">Lightning Round!</h2>
              <TimerRing duration={5} accentColor="#ef4444" onComplete={onNextRound} isLightning={true} />
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
                  accentColor="#ef4444"
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

          <motion.div
            className="glass-card p-6 border-2 border-red-500/30"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3, delay: 0.1 }}
            style={{ boxShadow: "0 0 20px rgba(239, 68, 68, 0.2)" }}
          >
            <h2 className="text-xl font-heading font-bold mb-4 text-red-500">Crowd Meter</h2>
            <CrowdMeter answers={crowdAnswers} accentColor="#ef4444" />

            <div className="mt-6 p-3 bg-red-500/20 rounded-lg border border-red-500/30">
              <div className="text-center font-bold text-red-400">DOUBLE POINTS!</div>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  )
}
