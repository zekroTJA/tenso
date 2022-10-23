import styled, { useTheme } from "styled-components";
import { useEffect, useState } from "react";

import { Floatbar } from "../../components/Floatbar";
import { Link } from "../../lib/tenso";
import { LinkEntry } from "../../components/LinkEntry";
import { ReactComponent as PlusIcon } from "../../assets/plus.svg";
import { useApi } from "../../hooks/useApi";
import { useNavigate } from "react-router-dom";

type Props = {};

const List = styled.div`
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1em;
  padding: 1em;
`;

const Container = styled.div`
  display: flex;
  justify-content: center;

  > ${List} {
    width: 100%;
    max-width: 50em;
  }
`;

export const MainRoute: React.FC<Props> = ({}) => {
  const fetch = useApi();
  const nav = useNavigate();
  const theme = useTheme();
  const [links, setLinks] = useState<Link[]>();

  const onLinkClick = (link: Link) => {
    nav("/" + link.id);
  };

  useEffect(() => {
    fetch((c) => c.links()).then((links) => setLinks(links));
  }, []);

  return (
    <Container>
      <List>
        {links?.map((l) => (
          <LinkEntry key={l.ident} link={l} onClick={onLinkClick} />
        ))}
      </List>

      <Floatbar
        buttons={[
          {
            content: (
              <>
                <PlusIcon />
                Add new link
              </>
            ),
            color: theme.accent,
            action: () => nav("/new"),
          },
        ]}
      />
    </Container>
  );
};
