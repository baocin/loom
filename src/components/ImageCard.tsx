import type React from "react"
import { useState } from "react"

interface ImageCardProps {
  data: {
    url: string
    caption: string
  }
}

const ImageCard: React.FC<ImageCardProps> = ({ data }) => {
  const [expanded, setExpanded] = useState(false)

  return (
    <div className="bg-white p-4 rounded shadow">
      <h3 className="text-lg font-semibold">Image</h3>
      <img
        src={data.url || "/placeholder.svg"}
        alt={data.caption}
        className={`w-full cursor-pointer transition-all duration-300 ${expanded ? "h-auto" : "h-32 object-cover"}`}
        onClick={() => setExpanded(!expanded)}
      />
      <p className="mt-2 text-sm text-gray-600">{data.caption}</p>
    </div>
  )
}

export default ImageCard

