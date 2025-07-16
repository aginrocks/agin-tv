import { carouselData } from "@/types/api-simple";
import { useEffect, useRef, useState } from "react";
import Controls from "./controls";
import { cva } from "class-variance-authority";

interface CarouselProps {
  slides?: carouselData;
}

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

export default function Carousel({ slides }: CarouselProps) {
  const [activeSlide, setSlide] = useState(0);

  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  const [prevSlide, setPrevSlide] = useState(activeSlide);
  const [animate, setAnimate] = useState(false);

  function animationHandler(slideValue: number) {
    if (prevSlide == slideValue) {
      return;
    }
    setAnimate(true);
    const slideTimeout = setTimeout(() => setSlide(slideValue), 200);
    const animationTimeout = setTimeout(() => setAnimate(false), 300);
    setPrevSlide(slideValue);

    return () => {
      clearTimeout(slideTimeout);
      clearTimeout(animationTimeout);
    };
  }

  function slideChange(right?: boolean, e?: React.MouseEvent) {
    e?.stopPropagation();
    if (slides == undefined) return;

    if (right == true) {
      if (activeSlide === slides.length - 1) {
        animationHandler(0);
      } else {
        animationHandler(activeSlide + 1);
      }
    } else {
      if (activeSlide === 0) {
        animationHandler(slides.length - 1);
      } else {
        animationHandler(activeSlide - 1);
      }
    }
  }

  useEffect(() => {
    if (slides == undefined) return;
    timeoutRef.current = setInterval(() => {
      slideChange(true);
    }, 10000);

    return () => {
      if (timeoutRef.current) {
        clearInterval(timeoutRef.current);
      }
    };
  }, [activeSlide, slides]);

  return (
    <div className="aspect-[16/7.5]  w-full overflow-hidden relative">
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
              Hitori Gotoh, a shy, awkward, and lonely high school student
              dreams of being in a band despite her doubts and worries, but when
              she is recruited to be the guitarist of a group looking to make it
              big, she realises her dream may be able to be fulfilled and come
              true.
            </div>
          </div>
          <Controls />
        </div>
      </div>
    </div>
  );
}
