import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

function IOButtions() {
  const outputFile = async () => {
    const path = await save({
      defaultPath: "output.csv"
    });
    if (path === null) {
      alert("キャンセルされました");
      return;
    }
    await invoke<void>("write_in_csv", { path }).then(() => {
      alert("ファイルに出力しました。");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです\n" + String(e));
    });
  }
  const inputFile = async () => {
    let selected: string | string[] | null = await open({
      multiple: false,
      directory: false, 
      filters: [{
        name: 'Text',
        extensions: ['csv']
      }]
    });
    if (selected === null || Array.isArray(selected)) {
      return;
    }
    await invoke<void>("read_from_csv", {fileName: selected}).then(() => {
      alert("ファイルから入力しました。");
    }, (e) => {
      console.log(e);
      alert("エラーが発生しました。エラーメッセージは以下の通りです\n" + String(e));
    });
  }

  return (
    <div className="io-container">
      <button type="button" className="io-button" onClick={outputFile}>ファイル出力</button>
      <button type="button" className="io-button" onClick={inputFile}>ファイル入力</button>
    </div>
  )
}

export default IOButtions;