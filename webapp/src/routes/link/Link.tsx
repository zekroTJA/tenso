import { Count, Link, Stats } from "../../lib/tenso";
import styled, { useTheme } from "styled-components";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

import { ReactComponent as ArrowRightIcon } from "../../assets/arrow-right.svg";
import { ReactComponent as BackIcon } from "../../assets/back.svg";
import { ReactComponent as ExtLinkIcon } from "../../assets/ext-link.svg";
import { ReactComponent as EyeCheckIcon } from "../../assets/eye-check.svg";
import { ReactComponent as EyeOffIcon } from "../../assets/eye-off.svg";
import { Floatbar } from "../../components/Floatbar";
import { ReactComponent as PencilIcon } from "../../assets/pencil.svg";
import ReactApexChart from "react-apexcharts";
import { ReactComponent as TrashIcon } from "../../assets/trash.svg";
import { useApi } from "../../hooks/useApi";

type Props = {};

const defaultChartOptions: Partial<ApexCharts.ApexOptions> = {
  theme: {
    mode: "dark",
  },
  chart: {
    id: "some-chart",
    toolbar: {
      show: false,
    },
    selection: {
      enabled: false,
    },
    zoom: {
      enabled: false,
    },
    background: "transparent",
  },
};

const Container = styled.div`
  padding: 1em;
  max-width: 50em;
  margin: 0 auto;
`;

const Destination = styled.div`
  display: flex;
  gap: 0.5em;
  align-items: center;
  margin-bottom: 1em;

  > a {
    color: ${(p) => p.theme.cyan};
    text-decoration: underline;
  }
`;

const Property = styled.section`
  display: flex;
  gap: 0.5em;
  align-items: center;
  margin-top: 0.5em;
`;

const Interactions = styled.div`
  width: 100%;
  background-color: ${(p) => p.theme.background3};
  padding: 0.5em 1em;
  border-radius: 8px;
  margin-top: 2em;
`;

const Chart = styled(ReactApexChart)`
  margin-top: 1em;
`;

export const LinkRoute: React.FC<Props> = ({}) => {
  const theme = useTheme();
  const { id } = useParams();
  const fetch = useApi();
  const nav = useNavigate();
  const [link, setLink] = useState<Link>();
  const [stats, setStats] = useState<Stats>();
  const [count, setCount] = useState<Count>();

  const deleteLink = () => {
    if (!id) return;
    fetch((c) => c.linkDelete(id)).then(() => nav(-1));
  };

  useEffect(() => {
    if (!id) return;
    fetch((c) => c.link(id)).then(setLink);
    fetch((c) => c.stats(id)).then(setStats);
    fetch((c) => c.count(id)).then(setCount);
  }, [id]);

  return (
    <div>
      <Container>
        {link && (
          <>
            <h2>{link.ident}</h2>
            <Destination>
              <ArrowRightIcon />
              <a href={link.destination} target="_blank">
                {link.destination}
              </a>
            </Destination>
            {(link.enabled && (
              <Property>
                <EyeCheckIcon />
                <span>Enabled</span>
              </Property>
            )) || (
              <Property>
                <EyeOffIcon />
                <span>Disabled</span>
              </Property>
            )}
            <Property>
              <ExtLinkIcon />
              <span>
                {link.permanent_redirect
                  ? "Permanent Redirect"
                  : "Temporary Redirect"}
              </span>
            </Property>
          </>
        )}
        {count && (
          <Interactions>{count.count} Interactions since creation</Interactions>
        )}
        {stats && (
          <>
            <Chart
              options={{
                ...defaultChartOptions,
                colors: [theme.accent],
                xaxis: {
                  categories: stats.map((s) => new Date(s[0]).toLocaleString()),
                },
              }}
              type="line"
              series={[
                {
                  name: "Interactions",
                  data: stats.map((s) => s[1]),
                },
              ]}
            />
          </>
        )}
      </Container>

      <Floatbar
        buttons={[
          {
            content: (
              <>
                <BackIcon />
                Back
              </>
            ),
            color: theme.orange,
            action: () => nav(-1),
          },
          {
            content: (
              <>
                <PencilIcon />
                Edit
              </>
            ),
            color: theme.accent,
            action: () => nav(`/${id}/edit`),
          },
          {
            content: (
              <>
                <TrashIcon />
                Delete
              </>
            ),
            color: theme.red,
            action: deleteLink,
          },
        ]}
      />
    </div>
  );
};
