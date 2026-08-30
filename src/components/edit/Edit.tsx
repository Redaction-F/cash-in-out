import { useRef, useState } from "react";
import EditButtons from "./EditButtons";
import Inputs from "./Inputs";
import { CashIORecord, DisplayHandler, Global } from "../../logic";
import { InputsRef, ModeOfEdit } from "./logic";

// edit ディスプレイ
// 出入金レコードの入力受付
function Edit(props: {
  displayHandler: DisplayHandler, 
  global: Global
}) {
  // modeの変更
  const setModeWrapper = (value: ModeOfEdit) => {
    setMode(value);
  };
  // 出入金データ新規作成開始
  const startCreate = () => {
    setModeWrapper("create");
  };
  // 出入金データ編集開始
  const startEdit = async (id: number | null) => {
    if (id === null) {
      alert("IDを入力して下さい。");
      return;
    }
    const records: CashIORecord | null = await CashIORecord.getById(id);
    if (records === null) {
      alert("入力されたIDのデータは存在しません。\n存在するデータのIDを入力するか、データ一覧から選択して編集してください。");
      return;
    }
    inputsRef.current!.set(records);
    setModeWrapper("update");
  };
  // 出入金データ新規作成
  const doCreate = async () => {
    const newData: CashIORecord = inputsRef.current!.get();
    await newData.create().then(() => {
      alert("新規作成が完了しました。");
      inputsRef.current!.setEmpty();
      setModeWrapper("select");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  };
  // 出入金データ更新
  const doUpdate = async () => {
    const changedData: CashIORecord = inputsRef.current!.get!();
    await changedData.update().then(() => {
      alert("編集が完了しました。");
      inputsRef.current!.setEmpty();
      setModeWrapper("select");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです。\n" + String(e));
    })
  };
  // 出入金データ編集中止
  const cancelEdit = () => {
    inputsRef.current!.setEmpty();
    setModeWrapper("select");
  };
  // IDフォームの内容から編集開始
  const startEditById = () => {
    startEdit(inputsRef.current!.getId());
  };

  // 現在のモード
  // useState: 入力フォームの再レンダリング
  const [mode, setMode] = useState<ModeOfEdit>("select");
  // 入力フォームの出入力管理
  const inputsRef = useRef<InputsRef>(null);

  // このdisplayに遷移時の処理
  props.displayHandler.onOpen = async(): Promise<void> => {
    await inputsRef.current!.reload();
  };
  // このdisplayから遷移時の処理
  props.displayHandler.onClose = async(): Promise<boolean> => {
    if (mode === "select") {
      return true;
    }
    if (await confirm("編集を中止しますか？\n編集中のデータは破棄され、このデータは変更されません。")) {
      cancelEdit();
      return true;
    } else {
      return false;
    }
  };
  // spectionFunctionを設定
  props.global.startEdit = startEdit;
  props.global.startCreate = startCreate;
  
  return (
    <>
      {/* ボタン群 */}
      <EditButtons mode={mode} onStartEdit={startEditById} onEdit={doUpdate} onStartCreate={startCreate} onCreate={doCreate} onCancel={cancelEdit}/>
      {/* 入力フォーム群 */}
      <Inputs mode={mode} startEditById={startEditById} ref={inputsRef}/>
    </>
  )
}

export default Edit;