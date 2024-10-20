function OptionButtons(props: {checkedRowsCount: number}) {
  return(
    <div className="option-buttons">
      <button className="option-button" disabled={props.checkedRowsCount !== 1}>編集</button>
      <button className="option-button" disabled={props.checkedRowsCount === 0}>削除</button>
    </div>
  )
}

export default OptionButtons;