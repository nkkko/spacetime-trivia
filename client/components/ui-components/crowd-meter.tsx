"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"

interface CrowdMeterProps {
  answers: number[]
  accentColor: string
}

export function CrowdMeter({ answers, accentColor }: CrowdMeterProps) {
  const [counts, setCounts] = useState(answers)

  // Simulate real-time updates
  useEffect(() => {
    const interval = setInterval(() => {
      setCounts((prev) => {
        return prev.map((count) => {
          const change = Math.floor(Math.random() * 5) - 1 // Random change between -1 and +3
          return Math.max(0, count + change)
        })
      })
    }, 120) // 120ms debounce as specified

    return () => clearInterval(interval)
  }, [])

  const maxCount = Math.max(...counts, 1)

  return (
    <div className="flex items-end h-32 gap-2">
      {counts.map((count, index) => {
        const height = `${Math.max(5, (count / maxCount) * 100)}%`

        return (
          <div key={index} className="relative w-16 flex flex-col items-center">
            <div className="text-xs mb-1 opacity-70">{count}</div>
            <div className="w-full bg-slate-800/50 rounded-md h-full">
              <motion.div
                className="crowd-meter-bar"
                style={{
                  height,
                  backgroundColor: `${accentColor}${index === 0 ? "FF" : index === 1 ? "CC" : index === 2 ? "99" : "66"}`,
                }}
                animate={{ height }}
                transition={{ duration: 0.12 }}
              />
            </div>
            <div className="mt-1 text-xs font-bold">{String.fromCharCode(65 + index)}</div>
          </div>
        )
      })}
    </div>
  )
}
