type buttonTypes = {
   buttonName: string;
   openProgram: (invocation: string) => void;
   invocation: string;
};

export default function Button({ buttonName, openProgram, invocation }: buttonTypes) {
  return (
      <button onClick={() => openProgram(invocation)}>
         {buttonName}
      </button>
  );
};