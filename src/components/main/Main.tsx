import Graph from "./Graph";

function Main() {
  return (
    <div>
      <Graph datas={[
        {
          category: "Category1",
          color: "#3f8f8f",
          amount: 1000
        },
        {
          category: "Category2",
          color: "#3faf4f",
          amount: 1000
        },
        {
          category: "Category3",
          color: "#af3f2f",
          amount: 2000
        },
      ]}/>
    </div>
  );
}

export default Main;