import { carouselData } from '@/types/api-simple';
import { useEffect, useRef, useState } from 'react';
import Controls from './controls';
import { cva } from 'class-variance-authority';
import { useRouter } from '@tanstack/react-router';
import SearchBar from '@components/ui/searchbar';
import SearchResults from '@components/SearchResults';
import { components } from '@/types/api';
import { apiUrl } from '@/config';
import { tokenAtom } from '@lib/atoms/token';
import { useAtomValue } from 'jotai';

interface CarouselProps {
  slides?: carouselData;
}

const imageStyle = cva(
  'w-full h-full object-cover opacity-1, transition-all duration-200 ease-in-out',
  {
    variants: {
      animate: { true: 'opacity-0 ' },
    },
  }
);

const slideDescription = cva(
  'max-w-full flex md:max-w-1/2 flex-col items-start justify-end z-2 gap-3 transition-all duration-200 ease-in-out',
  {
    variants: {
      padding: {
        true: 'gap-5',
      },
      animate: {
        true: 'translate-y-[10%] opacity-0',
      },
    },
  }
);

export default function Carousel({ slides }: CarouselProps) {
  const [activeSlide, setSlide] = useState(0);

  const timeoutRef = useRef<NodeJS.Timeout | null>(null);
  const searchTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  const [prevSlide, setPrevSlide] = useState(activeSlide);
  const [animate, setAnimate] = useState(false);

  const [searchQuery, setSearchQuery] = useState('');
  const [debouncedQuery, setDebouncedQuery] = useState('');
  const [searchResults, setSearchResults] = useState<components['schemas']['Movie'][]>([]);
  const [isSearching, setIsSearching] = useState(false);

  const token = useAtomValue(tokenAtom);
  const router = useRouter();

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

  // Debounce search query
  useEffect(() => {
    if (searchTimeoutRef.current) {
      clearTimeout(searchTimeoutRef.current);
    }

    searchTimeoutRef.current = setTimeout(() => {
      setDebouncedQuery(searchQuery);
    }, 500);

    return () => {
      if (searchTimeoutRef.current) {
        clearTimeout(searchTimeoutRef.current);
      }
    };
  }, [searchQuery]);

  // Fetch search results
  useEffect(() => {
    if (!debouncedQuery.trim()) {
      setSearchResults([]);
      setIsSearching(false);
      return;
    }

    const fetchSearchResults = async () => {
      setIsSearching(true);
      try {
        const response = await fetch(
          `${apiUrl}/api/search?query=${encodeURIComponent(debouncedQuery)}`,
          {
            headers: {
              Authorization: `Bearer ${token}`,
            },
          }
        );

        if (response.ok) {
          const data = await response.json();
          setSearchResults(data);
        } else {
          console.error('Search failed:', response.statusText);
          setSearchResults([]);
        }
      } catch (error) {
        console.error('Search error:', error);
        setSearchResults([]);
      } finally {
        setIsSearching(false);
      }
    };

    fetchSearchResults();
  }, [debouncedQuery, token]);

  const handleSearchChange = (query: string) => {
    setSearchQuery(query);
  };

  const showSearchResults = searchQuery.trim().length > 0;

  return (
    <div className="w-full">
      <div className="w-full bg-background" onClick={(e) => e.stopPropagation()}>
        <SearchBar onSearchChange={handleSearchChange} value={searchQuery} />
      </div>

      {showSearchResults ? (
        <SearchResults results={searchResults} isLoading={isSearching} />
      ) : (
        <div
          onClick={() => router.navigate({ to: `/app/movie/${slides?.[activeSlide].tmdb_id}` })}
          className="aspect-[16/7.5] w-full overflow-hidden relative cursor-pointer"
        >
          <div className="w-full h-full relative">
            <img
              className={imageStyle({ animate })}
              src={slides?.[activeSlide]?.background_url ?? undefined}
            />
            <div className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-t from-background to-transparent" />
            <div className="absolute left-0 right-0 bottom-0 flex justify-between items-end p-5 px-7">
              <div className={slideDescription({ animate })}>
                <div className="text-5xl font-bold">{slides?.[activeSlide]?.name}</div>
                <div className="text-lg text-muted-foreground line-clamp-5">
                  {slides?.[activeSlide]?.description}
                </div>
              </div>
              <Controls
                onLeftClick={(e) => slideChange(false, e)}
                onRightClick={(e) => slideChange(true, e)}
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
