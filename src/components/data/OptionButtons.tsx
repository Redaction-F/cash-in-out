import { forwardRef, useImperativeHandle, useState } from "react";
import { CashIORecord, Global } from "../../logic";
import { OptionButtonsRef } from "./logic";

// ボタン等
const OptionButtons = forwardRef((props: {
  reload: () => Promise<void>, 
  getCheckedIds: (() => number[]) | undefined,
  global: Global
}, ref: React.ForwardedRef<OptionButtonsRef>) => {
  // 編集タブに遷移し、編集開始
  const doEdit = async () => {
    if (await props.global.changeDisplay!("edit")) {
      const checkedId: number[] = props.getCheckedIds!();
      await props.global.startEdit!(checkedId.length === 0 ? null : checkedId[0]);
    };
  };
  // 編集タブに遷移し、新規作成開始
  const doCreate = async () => {
    if (await props.global.changeDisplay!("edit")) {
      props.global.startCreate!();
    };
  }
  // データを削除
  const doDelete = async () => {
    if (!confirm("削除しますか？")) {
      return;
    }
    const checkedId: number[] = props.getCheckedIds!();
    const errors: string[] = [];
    for (const v of checkedId) {
      await CashIORecord.deleteById(v).then(() => {}, (e) => {
        errors.push(String(e));
      });
    }
    if (errors.length === 0) {
      alert("データを削除しました。");
    } else {
      console.log(errors.join("\n"));
      alert("エラーが発生しました。エラーメッセージは以下の通りです。 \n" + errors.join("\n"));
    }
    props.reload();
  }

  // 表でチェックされている行の数
  // useState: ボタンのdisabledの切り替え
  const [checkedCount, setCheckedCount] = useState<number>(0);

  // optionButtonsRefの初期化
  useImperativeHandle(ref, () => ({
    onUpdateCheckBoxes: (checkedCount: number) => setCheckedCount(checkedCount)
  }));

  return(
    <div className="option-buttons">
      <button type="button" className="option-button" onClick={doCreate} disabled={checkedCount !== 0}>新規</button>
      <button type="button" className="option-button" onClick={doEdit} disabled={checkedCount !== 1}>編集</button>
      <button type="button" className="option-button" onClick={doDelete} disabled={checkedCount === 0}>削除</button>
    </div>
  )
})

export default OptionButtons;