"use client"

import { motion } from "framer-motion"
import { Zap } from "lucide-react"

interface LightningBannerProps {
  visible: boolean
}

export function LightningBanner({ visible }: LightningBannerProps) {
  if (!visible) return null

  return (
    <motion.div
      className="fixed top-0 left-0 right-0 z-50 flex justify-center"
      initial={{ y: -100 }}
      animate={{ y: 0 }}
      exit={{ y: -100 }}
      transition={{ type: "spring", stiffness: 300, damping: 25 }}
    >
      <div className="lightning-banner flex items-center gap-2 mt-4">
        <Zap className="animate-pulse" size={20} />
        <span>LIGHTNING ROUND! DOUBLE POINTS!</span>
        <Zap className="animate-pulse" size={20} />
      </div>
    </motion.div>
  )
}
