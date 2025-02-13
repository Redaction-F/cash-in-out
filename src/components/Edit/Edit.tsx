function Edit() {
  return (
    <>
      <div className="edit-buttons">
        <button className="edit-button">編集</button>
        <button className="edit-button" disabled>変更</button>
        <button className="edit-button" disabled>キャンセル</button>
      </div>
      <div className="inputs-wrapper">
        <div className="input-row">
          <label className="input-label" htmlFor="id">id</label>
          <input type="number" id="id" className="input-input" />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="date">日時</label>
          <input type="text" id="date" className="input-input" disabled />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="category">カテゴリ</label>
          <input type="text" id="category" className="input-input" disabled />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="title">タイトル</label>
          <input type="text" id="title" className="input-input" disabled />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="amount">金額</label>
          <input type="text" id="amount" className="input-input" disabled />
        </div>
        <div className="input-row">
          <label className="input-label" htmlFor="memo">メモ</label>
          <textarea className="input-textarea" disabled />
        </div>
      </div>
    </>
  )
}

export default Edit;