const Description = ({ tasks = [], handleSub }) => {
  return (
    <div className="description">
      <div className="content">{tasks[0]?.description}</div>
      <div className="submit">
        <button
          onClick={(e) => {
            if (e.defaultPrevented) return;
            e.preventDefault();
            e.stopPropagation();
            handleSub();
          }}
        >
          Submit
        </button>
      </div>
    </div>
  );
};

export default Description;
