import type React from "react"
import type { TimelineItem } from "../types"
import SensorData from "./SensorData"
import Note from "./Note"
import ImageCard from "./ImageCard"
import Link from "./Link"

interface TimelineEventProps {
  item: TimelineItem
  isLast: boolean
}

const TimelineEvent: React.FC<TimelineEventProps> = ({ item, isLast }) => {
  const renderEventContent = () => {
    switch (item.type) {
      case "sensor":
        return <SensorData data={item.data} />
      case "note":
        return <Note data={item.data} />
      case "image":
        return <ImageCard data={item.data} />
      case "link":
        return <Link data={item.data} />
      default:
        return null
    }
  }

  return (
    <div className={`flex items-start mb-8 ${isLast ? "pb-8" : ""}`}>
      <div className="relative flex-shrink-0">
        <div className="absolute left-0 top-1/2 -translate-x-1/2 -translate-y-1/2 w-4 h-4 bg-black rounded-full z-10"></div>
      </div>
      <div className="flex-grow pl-6">
        <div className="w-full max-w-2xl">
          {renderEventContent()}
          <div className="text-xs text-gray-500 mt-2">{item.date.toLocaleDateString()}</div>
        </div>
      </div>
    </div>
  )
}

export default TimelineEvent

