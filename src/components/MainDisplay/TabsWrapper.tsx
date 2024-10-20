function TabsWrapper(props: {changeDisplayWrapper: (tab_name: string) => () => void}) {
  return (
    <div className="tabs-wrapper">
      <div className="tab">
        <input id="tab-main" className="tab-button" type="radio" name="tab" value="main" onChange={props.changeDisplayWrapper("main")} defaultChecked />
        <label className="tab-label" htmlFor="tab-main">
          メイン
        </label>
      </div>
      <div className="tab">
        <input id="tab-data" className="tab-button" type="radio" name="tab" value="data" onChange={props.changeDisplayWrapper("data")} />
        <label className="tab-label" htmlFor="tab-data">
          データ
        </label>
      </div>
      <div className="tab">
        <input id="tab-edit" className="tab-button" type="radio" name="tab" value="edit" onChange={props.changeDisplayWrapper("edit")} />
        <label className="tab-label" htmlFor="tab-edit">
          編集
        </label>
      </div>
      <div className="tab">
        <input id="tab-set" className="tab-button" type="radio" name="tab" value="set" onChange={props.changeDisplayWrapper("set")} />
        <label className="tab-label" htmlFor="tab-set">
          設定
        </label>
      </div>
    </div>
  )
}

export default TabsWrapper;