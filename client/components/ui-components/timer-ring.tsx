"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"

interface TimerRingProps {
  duration: number // in seconds
  accentColor: string
  onComplete?: () => void
  isLightning?: boolean
}

export function TimerRing({ duration, accentColor, onComplete, isLightning = false }: TimerRingProps) {
  const [timeLeft, setTimeLeft] = useState(duration)
  const [isRunning, setIsRunning] = useState(true)

  useEffect(() => {
    if (!isRunning) return

    const interval = setInterval(() => {
      setTimeLeft((prev) => {
        if (prev <= 0) {
          clearInterval(interval)
          setIsRunning(false)
          onComplete?.()
          return 0
        }
        return prev - 0.1
      })
    }, 100)

    return () => clearInterval(interval)
  }, [isRunning, onComplete])

  const progress = (timeLeft / duration) * 100
  const circumference = 2 * Math.PI * 45 // 45 is the radius
  const strokeDashoffset = circumference * (1 - progress / 100)

  // Color changes as timer progresses
  const getStrokeColor = () => {
    if (isLightning) {
      // Lightning round is always red-based
      return timeLeft < 3 ? "#ef4444" : "#f97316"
    }

    if (timeLeft < 3) return "#ef4444" // Red for last 3 seconds
    if (timeLeft < 6) return "#f97316" // Orange for 3-6 seconds
    return accentColor // Default accent color
  }

  return (
    <div className="relative w-24 h-24 flex items-center justify-center">
      <svg className="w-full h-full" viewBox="0 0 100 100">
        {/* Background circle */}
        <circle cx="50" cy="50" r="45" fill="none" stroke="rgba(255, 255, 255, 0.1)" strokeWidth="6" />

        {/* Timer progress */}
        <motion.circle
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke={getStrokeColor()}
          strokeWidth="6"
          strokeDasharray={circumference}
          strokeDashoffset={strokeDashoffset}
          strokeLinecap="round"
          transform="rotate(-90 50 50)"
          className="timer-ring"
          initial={{ strokeDashoffset: 0 }}
          animate={{ strokeDashoffset }}
          transition={{ duration: 0.1, ease: "linear" }}
        />
      </svg>

      <div className="absolute inset-0 flex items-center justify-center">
        <span className="text-2xl font-heading font-bold">{Math.ceil(timeLeft)}</span>
      </div>
    </div>
  )
}
