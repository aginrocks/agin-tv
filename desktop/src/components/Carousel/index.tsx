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
    console.log(slides);

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
          className={imageStyle({ animate })}
          src={slides?.[activeSlide]?.background_url ?? undefined}
        />
        <div className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-t from-background to-transparent" />
        <div className="absolute left-0 right-0 bottom-0 flex justify-between items-end">
          <div className={slideDescription({ animate })}>
            <div className="text-5xl font-bold">
              {slides?.[activeSlide]?.name}
            </div>
            <div className="text-lg text-muted-foreground line-clamp-5">
              {slides?.[activeSlide]?.description}
            </div>
          </div>
          <Controls />
        </div>
      </div>
    </div>
  );
}
