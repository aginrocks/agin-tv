import { cva } from "class-variance-authority";
import Controls from "../controls";

const imageStyle = cva(
  "w-full h-full object-cover opacity-1, transition-all duration-200 ease-in-out",
  {
    variants: {
      animate: { true: "opacity-0 " },
    },
  }
);

const slideDescription = cva(
  "max-w-full p-[3%] flex md:max-w-1/2 flex-col items-start justify-end z-2 gap-3 transition-all duration-200 ease-in-out",
  {
    variants: {
      padding: {
        true: "gap-5",
      },
      animate: {
        true: "translate-y-[10%] opacity-0",
      },
    },
  }
);

export default function Slide() {
  return (
    <div className="w-full h-full relative">
      <img
        className={imageStyle({ animate: false })}
        src="https://sm.ign.com/t/ign_pl/screenshot/default/bocchi-blogroll-1672352846343_3u73.1280.jpg"
      />
      <div className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-t from-background to-transparent" />
      <div className="absolute left-0 right-0 bottom-0 flex justify-between items-end">
        <div className={slideDescription({})}>
          <div className="text-5xl font-bold">Bocchi the Rock</div>
          <div className="text-lg text-muted-foreground line-clamp-5">
            Hitori Gotoh, a shy, awkward, and lonely high school student dreams
            of being in a band despite her doubts and worries, but when she is
            recruited to be the guitarist of a group looking to make it big, she
            realises her dream may be able to be fulfilled and come true.
          </div>
        </div>
        <Controls />
      </div>
    </div>
  );
}
