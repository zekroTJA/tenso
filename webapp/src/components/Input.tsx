import Color from "color";
import styled from "styled-components";

export const Input = styled.input`
  background-color: ${(p) => p.theme.background2};
  border: none;
  padding: 0.5em;
  border-radius: 8px;
  color: inherit;
  font-size: 1em;
  outline: solid 3px ${(p) => Color(p.theme.accent).fade(1).hexa()};

  transition: all 0.15s ease;

  &:focus {
    outline-color: ${(p) => p.theme.accent};
  }
`;
