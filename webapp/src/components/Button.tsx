import Color from "color";
import styled from "styled-components";

type Props = {
  color?: string;
};

export const Button = styled.button<Props>`
  border: none;
  background-color: ${(p) =>
    Color(p.color ?? p.theme.accent)
      .fade(0.2)
      .hexa()};
  cursor: pointer;
  padding: 0.5em;
  font-size: 1em;
  outline: solid 3px ${(p) => Color(p.theme.accent).fade(1).hexa()};
  color: inherit;
  border-radius: 8px;

  transition: all 0.15s ease;

  &:focus {
    outline-color: ${(p) => p.theme.accent};
  }

  &:hover {
    background-color: ${(p) => p.color ?? p.theme.accent};
  }
`;
