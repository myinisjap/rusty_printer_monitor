function ProgressBar(props) {
  const barStyling = {
    border: ".1em solid",
    borderRadius: "1em",
    height: "1em",
    margin: "1em"
  }
  if (isNaN(Number(props.progress))) {
    return (
      <div style={barStyling}>
        {props.progress}
      </div>
    )
  }
  return (
    <div style={barStyling}>
      <div style={{
        width: (props.progress + "%"),
        backgroundColor: "lightgreen",
        color: "black",
        height: "1em",
        borderRadius: "1em"
      }}>
        {props.progress + "%"}
      </div>
    </div>
  )
}

export default ProgressBar