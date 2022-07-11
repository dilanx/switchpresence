import { invoke } from '@tauri-apps/api/tauri';
import { useState, useTransition } from 'react';
import styled from 'styled-components';

import games from './games.json';

interface Game {
  title: string;
  image?: string;
}

const ImageContainer = styled.div`
  aspect-ratio: 16/9;
  max-width: 240px;
  display: block;
  margin: 50px auto;
  border-radius: 8px;
  box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px;
  overflow: hidden;
`;

const Image = styled.img`
  width: 100%;
`;

const NowPlaying = styled.p`
  font-size: 16px;
  margin: 10px 0;
  text-align: center;
  color: #737373;
  font-weight: bold;
`;

const Title = styled.p`
  font-size: 32px;
  color: #f0f0f0;
  font-weight: bold;
  text-align: center;
  margin: 0 40px 48px 40px;
`;

const Button = styled.button`
  border: none;
  display: block;
  margin: 20px auto;
  font-size: 16px;
  font-weight: bold;
  color: #a3a3a3;
  background-color: #404040;
  width: 240px;
  height: 40px;
  border-radius: 8px;

  &:hover {
    background-color: #484848;
    color: #f0f0f0;
  }

  &:focus {
    background-color: #525252;
    color: white;
  }
`;

const Input = styled.input`
  display: block;
  margin: 10px auto;
  width: 240px;
  padding: 8px;
  outline: none;
  border: 2px solid #404040;
  border-radius: 8px;
  background-color: #262626;
  color: #f0f0f0;
  font-size: 16px;

  &:hover {
    border-color: #525252;
  }

  &:focus {
    border-color: #737373;
  }
`;

const SearchResults = styled.ul`
  list-style: none;
  display: block;
  margin: 0 auto;
  padding: 0;
  width: 300px;
  height: 180px;
  border: 2px solid #404040;
  border-radius: 8px;
  overflow: hidden;
  overflow-y: scroll;
`;

const SearchResultItem = styled.button`
  background-color: #171717;
  color: #d4d4d4;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  font-size: 14px;
  width: 100%;
  border: none;
  outline: none;
  margin: 0;
  padding: 4px 8px;
  text-align: left;

  &:hover {
    background-color: #262626;
  }

  &:focus {
    background-color: #323232;
    color: #f0f0f0;
  }
`;

const SearchMessage = styled.p`
  font-size: 16px;
  color: #525252;
  text-align: center;
  margin: 0;
  padding: 4px 8px;
  font-weight: bold;
`;

function App() {
  const [game, setGame] = useState<Game | null>(null);
  const [editMode, setEditMode] = useState(false);
  const [searchResults, setSearchResults] = useState<JSX.Element[]>([]);
  const [_, startTransition] = useTransition();

  function updateQuery(text: string) {
    startTransition(() => {
      setSearchResults(
        getSearchResults(text, (game: Game) => {
          setGame(game);
          setEditMode(false);
          invoke('update_presence', {
            game: game.title,
          });
        })
      );
    });
  }

  return (
    <div>
      <ImageContainer>
        <Image src={game?.image ?? ''} />
      </ImageContainer>
      <NowPlaying>Now Playing</NowPlaying>
      {editMode ? (
        <>
          <Input
            autoFocus={true}
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck={false}
            onChange={(event) => {
              updateQuery(event.target.value);
            }}
          />
          <SearchResults>{searchResults}</SearchResults>
          <Button
            onClick={() => {
              setEditMode(false);
            }}
          >
            Cancel
          </Button>
        </>
      ) : (
        <>
          <Title>{game ? game.title : 'Nothing'}</Title>
          <Button
            onClick={() => {
              setEditMode(true);
              setSearchResults(getSearchResults(''));
            }}
          >
            Edit
          </Button>
        </>
      )}
    </div>
  );
}

function getSearchResults(query: string, select?: (game: Game) => void) {
  let results: JSX.Element[] = [];
  if (query.length === 0) {
    return [
      <li key="no-query">
        <SearchMessage>Enter a game title above</SearchMessage>
      </li>,
    ];
  }
  if (query.length < 3) {
    return [
      <li key="too-short">
        <SearchMessage>Keep typing...</SearchMessage>
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
            <SearchMessage>Results capped at 100</SearchMessage>
          </li>
        );
        break;
      }
      results.push(
        <li key={i}>
          <SearchResultItem
            onClick={() => {
              if (select) select(game);
            }}
          >
            {game.title}
          </SearchResultItem>
        </li>
      );
    }
  }
  results.push(
    <li key="manual">
      <SearchResultItem
        onClick={() => {
          if (select) select({ title: query });
        }}
      >
        Use '{query}'
      </SearchResultItem>
    </li>
  );
  return results;
}

export default App;
