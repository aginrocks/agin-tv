import { IconSearch } from '@tabler/icons-react';

interface SearchBarProps {
  onSearchChange?: (query: string) => void;
  value?: string;
}

export default function SearchBar({ onSearchChange, value }: SearchBarProps) {
  return (
    <div className="w-full p-3 border-b border-muted-foreground/10">
      <div className="flex items-center gap-2 px-3 py-2 bg-muted/30 rounded-lg border border-muted-foreground/20 focus-within:border-primary/50 focus-within:bg-muted/50 transition-colors">
        <IconSearch className="w-5 h-5 text-muted-foreground flex-shrink-0" />
        <input
          type="text"
          placeholder="Type something to search"
          className="flex-1 bg-transparent outline-none text-sm text-foreground placeholder:text-muted-foreground"
          value={value}
          onChange={(e) => onSearchChange?.(e.target.value)}
        />
      </div>
    </div>
  );
}
