"use client"

import { useState } from "react"
import { motion } from "framer-motion"
import { Check, X } from "lucide-react"

interface AnswerButtonProps {
  text: string
  index: number
  accentColor: string
  isSelected?: boolean
  isCorrect?: boolean | null
  isWrong?: boolean | null
  isDisabled?: boolean
  onClick: () => void
}

export function AnswerButton({
  text,
  index,
  accentColor,
  isSelected = false,
  isCorrect = null,
  isWrong = null,
  isDisabled = false,
  onClick,
}: AnswerButtonProps) {
  const [shake, setShake] = useState(false)

  const handleClick = () => {
    if (isDisabled) return

    if (isWrong) {
      setShake(true)
      setTimeout(() => setShake(false), 500)
    }

    onClick()
  }

  const getButtonClasses = () => {
    let classes = "answer-button w-full text-left"

    if (isSelected) classes += " selected"
    if (isCorrect) classes += " correct"
    if (isWrong) classes += " wrong"
    if (isDisabled) classes += " opacity-60 cursor-not-allowed"
    if (shake) classes += " shake"

    return classes
  }

  const getButtonStyles = () => {
    if (isCorrect) {
      return {
        borderColor: "#22c55e",
        boxShadow: "0 0 15px rgba(34, 197, 94, 0.5)",
      }
    }

    if (isWrong) {
      return {
        borderColor: "#ef4444",
        boxShadow: "0 0 15px rgba(239, 68, 68, 0.5)",
      }
    }

    if (isSelected) {
      return {
        borderColor: accentColor,
        boxShadow: `0 0 15px ${accentColor}50`,
      }
    }

    return {
      borderColor: "rgba(255, 255, 255, 0.1)",
    }
  }

  const letters = ["A", "B", "C", "D"]

  return (
    <motion.button
      className={getButtonClasses()}
      style={getButtonStyles()}
      onClick={handleClick}
      disabled={isDisabled}
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2, delay: index * 0.1 }}
      whileHover={!isDisabled ? { scale: 1.02 } : {}}
      whileTap={!isDisabled ? { scale: 0.98 } : {}}
    >
      <div className="flex items-center gap-3">
        <div
          className="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold"
          style={{ backgroundColor: `${accentColor}40` }}
        >
          {letters[index]}
        </div>

        <span className="flex-1">{text}</span>

        {isCorrect && (
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ type: "spring", stiffness: 300, damping: 10 }}
          >
            <Check className="text-green-500" />
          </motion.div>
        )}

        {isWrong && (
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ type: "spring", stiffness: 300, damping: 10 }}
          >
            <X className="text-red-500" />
          </motion.div>
        )}
      </div>
    </motion.button>
  )
}
