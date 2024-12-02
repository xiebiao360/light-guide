export const format_timestamp = timestamp => {
  if (!timestamp) {
    return ''
  }
  const date = new Date(timestamp * 1000)
  return date.toLocaleString()
}
