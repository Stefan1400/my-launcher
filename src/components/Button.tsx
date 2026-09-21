type buttonTypes = {
  buttonName: string;
  openProgram: (invocation: string) => void;
  invocation: string;
};

export default function Button({
  buttonName,
  openProgram,
  invocation,
}: buttonTypes) {
  return (
    <button
      className={`launcher-button ${buttonName}`}
      onClick={() => openProgram(invocation)}
    >
      {buttonName}
    </button>
  );
}