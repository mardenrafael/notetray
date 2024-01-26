function Note() {
  return (
    <div className="bg-slate-900 text-slate-100 h-screen flex flex-col select-none ">
      <textarea
        placeholder="Sua anotação"
        className="bg-slate-900 text-slate-100 p-2 text-center focus:ring-0 focus:outline-none w-full h-full"
      ></textarea>
    </div>
  );
}

export default Note;
