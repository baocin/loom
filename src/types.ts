export interface TimelineItem {
  id: string
  date: Date
  type: "sensor" | "note" | "image" | "link"
  data: any
}

