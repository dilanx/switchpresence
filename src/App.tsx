import { message } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { useEffect, useState, useTransition } from 'react';
import { RingLoader } from 'react-spinners';

import './App.scss';

interface Game {
  title: string;
  image?: string;
}

function App() {
  const [game, setGame] = useState<Game | null>(null);
  const [editMode, setEditMode] = useState(false);
  const [searchResults, setSearchResults] = useState<JSX.Element[]>([]);
  const [_, startTransition] = useTransition();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    appWindow.listen('event_clear_presence', () => {
      setLoading(true);
      setGame(null);
      invoke('clear_presence').then((success) => {
        if (!success) {
          message('Failed to clear presence.');
        }
        setLoading(false);
      });
    });
  }, []);

  function updateQuery(text: string) {
    startTransition(() => {
      setSearchResults(
        getSearchResults(text, (game: Game) => {
          setLoading(true);
          setGame(game);
          setEditMode(false);
          invoke('update_presence', {
            game: game.title,
          }).then((success) => {
            if (!success) {
              message('Failed to update presence.');
            }
            setLoading(false);
          });
        })
      );
    });
  }

  return (
    <div>
      <div className="image-container">
        <img src={game?.image ?? ''} />
      </div>
      <p className="now-playing">Now Playing</p>
      {editMode ? (
        <>
          <input
            className="search-input"
            autoFocus={true}
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck={false}
            onChange={(event) => {
              updateQuery(event.target.value);
            }}
          />
          <div className="search-results">{searchResults}</div>
          <button
            className="regular"
            onClick={() => {
              setEditMode(false);
            }}
          >
            Cancel
          </button>
        </>
      ) : (
        <>
          <p className="title">{game ? game.title : 'Nothing'}</p>
          <button
            className="regular"
            disabled={loading}
            onClick={() => {
              setEditMode(true);
              setSearchResults(getSearchResults(''));
            }}
          >
            Edit
          </button>
        </>
      )}
      <RingLoader
        color="#EB459E"
        loading={loading}
        cssOverride={{
          position: 'absolute',
          top: '16px',
          right: '16px',
        }}
        size={32}
      />
    </div>
  );
}

function getSearchResults(query: string, select?: (game: Game) => void) {
  let results: JSX.Element[] = [];
  if (query.length === 0) {
    return [
      <li key="no-query">
        <p>Enter a game title above</p>
      </li>,
    ];
  }
  if (query.length < 3) {
    return [
      <li key="too-short">
        <p>Keep typing...</p>
      </li>,
    ];
  }
  for (let i = 0; i < games.length; i++) {
    let game = games[i];
    let title = game.title.toLowerCase().replace(/[^a-z0-9 ]/gi, '');
    if (title.includes(query.toLowerCase())) {
      if (results.length > 100) {
        results.push(
          <li key="too-many">
            <p>Results capped at 100</p>
          </li>
        );
        break;
      }
      results.push(
        <li key={i}>
          <button
            onClick={() => {
              if (select) select(game);
            }}
          >
            {game.title}
          </button>
        </li>
      );
    }
  }
  results.push(
    <li key="manual">
      <button
        onClick={() => {
          if (select) select({ title: query });
        }}
      >
        Use '{query}'
      </button>
    </li>
  );
  return results;
}

export default App;
