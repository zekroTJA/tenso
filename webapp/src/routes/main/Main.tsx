import styled, { useTheme } from "styled-components";
import { useCallback, useEffect, useState } from "react";

import { Floatbar } from "../../components/Floatbar";
import { Link } from "../../lib/tenso";
import { LinkEntry } from "../../components/LinkEntry";
import { ReactComponent as PlusIcon } from "../../assets/plus.svg";
import { SearchBar } from "../../components/SearchBar";
import { debounce } from "debounce";
import { useApi } from "../../hooks/useApi";
import { useNavigate } from "react-router-dom";

type Props = {};

const List = styled.div`
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1em;
  padding: 1em 0em 5em 0em;
`;

const Container = styled.div`
  max-width: 50em;
  width: 100%;
  display: flex;
  flex-direction: column;

  > ${List} {
    height: 100%;
  }
`;

export const MainRoute: React.FC<Props> = ({}) => {
  const fetch = useApi();
  const nav = useNavigate();
  const theme = useTheme();
  const [links, setLinks] = useState<Link[]>();
  const [search, setSearch] = useState<string>("");

  const onLinkClick = (link: Link) => {
    nav("/" + link.id);
  };

  const onSearch = useCallback(debounce(setSearch, 500), []);

  useEffect(() => {
    fetch((c) => c.links(search)).then((links) => setLinks(links));
  }, [search]);

  return (
    <Container>
      <SearchBar onValueChange={onSearch} />
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
