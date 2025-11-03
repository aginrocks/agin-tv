import { useRouter } from '@tanstack/react-router';
import { components } from '@/types/api';

interface SearchResultsProps {
  results: components['schemas']['Movie'][];
  isLoading?: boolean;
}

export default function SearchResults({ results, isLoading }: SearchResultsProps) {
  const router = useRouter();

  if (isLoading) {
    return (
      <div className="w-full p-8">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {[...Array(10)].map((_, i) => (
            <div key={i} className="aspect-[2/3] bg-muted/30 rounded-lg animate-pulse" />
          ))}
        </div>
      </div>
    );
  }

  if (results.length === 0) {
    return (
      <div className="w-full p-8 flex flex-col items-center justify-center text-center gap-2 min-h-[400px]">
        <p className="text-xl text-muted-foreground">No results found</p>
        <p className="text-sm text-muted-foreground/70">Try searching for something else</p>
      </div>
    );
  }

  return (
    <div className="w-full p-8">
      <h2 className="text-2xl font-bold mb-6">Search Results ({results.length})</h2>
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
        {results.map((movie) => (
          <div
            key={movie._id}
            onClick={() => router.navigate({ to: `/app/movie/${movie.tmdb_id}` })}
            className="group cursor-pointer"
          >
            <div className="relative aspect-[2/3] overflow-hidden rounded-lg bg-muted/30 border border-muted-foreground/10 transition-all duration-200 group-hover:scale-105 group-hover:border-primary/50 group-hover:shadow-lg">
              {movie.vertical_cover_url ? (
                <img
                  src={movie.vertical_cover_url}
                  alt={movie.name}
                  className="w-full h-full object-cover"
                />
              ) : (
                <div className="w-full h-full flex items-center justify-center">
                  <p className="text-muted-foreground text-sm text-center px-4">
                    No Image Available
                  </p>
                </div>
              )}
              <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                <div className="absolute bottom-0 left-0 right-0 p-3">
                  <p className="text-white font-semibold text-sm line-clamp-2">{movie.name}</p>
                  {movie.release_date && (
                    <p className="text-white/70 text-xs mt-1">
                      {new Date(movie.release_date).getFullYear()}
                    </p>
                  )}
                </div>
              </div>
            </div>
            <div className="mt-2 px-1">
              <p className="font-medium text-sm line-clamp-1">{movie.name}</p>
              {movie.genres && movie.genres.length > 0 && (
                <p className="text-xs text-muted-foreground line-clamp-1">
                  {movie.genres.map((g) => g.name).join(', ')}
                </p>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
