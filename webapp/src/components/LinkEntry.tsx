import styled, { css } from "styled-components";

import { Link } from "../lib/tenso";

type Props = {
  link: Link;
  onClick: (link: Link) => void;
};

const Container = styled.div<{ disabled: boolean }>`
  background-color: ${(p) => p.theme.background2};
  padding: 1em;
  border-radius: 12px;

  transition: all 0.15s ease;

  > h2 {
    font-weight: 300;
    margin-bottom: 0.6em;
    ${(p) =>
      p.disabled &&
      css`
        text-decoration: line-through;
        opacity: 0.75; ;
      `}
  }

  &:hover {
    transform: scale(1.03);
  }
`;

export const LinkEntry: React.FC<Props> = ({ link, onClick }) => {
  return (
    <Container onClick={() => onClick(link)} disabled={!link.enabled}>
      <h2>{link.ident}</h2>
      <span>{link.destination}</span>
    </Container>
  );
};
