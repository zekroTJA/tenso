import { ReactComponent as SearchIcon } from "../assets/search.svg";
import styled from "styled-components";

type Props = {
  value?: string;
  onValueChange?: (v: string) => void;
};

const Container = styled.div`
  display: flex;
  align-items: center;
  gap: 1em;
  width: 100%;
  padding: 0.7em 1em;
  border-radius: 12px;
  background-color: ${(p) => p.theme.background3};

  > input {
    width: 100%;
    background: none;
    border: none;
    outline: none;
    color: inherit;
    font-size: 1em;
  }
`;

export const SearchBar: React.FC<Props> = ({
  value,
  onValueChange = () => {},
}) => {
  return (
    <Container>
      <SearchIcon />
      <input
        value={value}
        onInput={(e) => onValueChange(e.currentTarget.value)}
      />
    </Container>
  );
};
