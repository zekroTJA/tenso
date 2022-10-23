import Color from "color";
import styled from "styled-components";
import { useId } from "react";

export type ButtonTile = {
  content: string | JSX.Element;
  color: string;
  action: () => void;
  disabled?: boolean;
};

type Props = {
  buttons: ButtonTile[];
};

const Container = styled.div`
  position: fixed;
  display: flex;
  bottom: 0em;
  width: 100%;
  padding: 1em;

  > div {
    display: flex;
    border-radius: 8px;
    overflow: hidden;
    margin: 0 auto;
    width: 100%;
    max-width: 30em;
  }
`;

const Button = styled.button<{ color: string }>`
  width: 200%;
  color: inherit;
  padding: 0.6em;
  border: none;
  outline: none;
  font-size: 1em;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5em;

  transition: all 0.2s ease;

  background: linear-gradient(
    90deg,
    ${(p) => p.color} 0%,
    ${(p) => Color(p.color).darken(0.2).hexa()}
  );

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
`;

export const Floatbar: React.FC<Props> = ({ buttons }) => {
  return (
    <Container>
      <div>
        {buttons.map((b) => (
          <Button
            key={useId()}
            color={b.color}
            onClick={b.action}
            disabled={b.disabled}
          >
            {b.content}
          </Button>
        ))}
      </div>
    </Container>
  );
};
