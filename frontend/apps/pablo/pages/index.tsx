import type { NextPage } from "next";
import { Box, Container, Grid } from "@mui/material";
import { PageTitle } from "@/components";
import { Statistics } from "@/components/Organisms/overview/Statistics";
import { useDotSamaContext } from "substrate-react";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { WalletBreakdownBox } from "@/components/Organisms/overview/WalletBreakdownBox";
import { LiquidityProvidersBox } from "@/components/Organisms/overview/LiquidityProvidersBox";
import { VolumeChart } from "@/components/Organisms/overview/VolumeChart";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";

const Home: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <PoolLayout>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle title="Overview" />
          </Box>

          <Grid container>
            {!connected && (
              <Grid item xs={12}>
                <ConnectWalletFeaturedBox
                  mt={8}
                  title="Connect wallet"
                  textBelow="To see your portfolio, connect your wallet."
                  ButtonProps={{ label: "Connect Wallet", size: "small" }}
                />
              </Grid>
            )}
            <Grid item xs={12} mt={8}>
              <Statistics />
            </Grid>
            <Grid container spacing={8}>
              <Grid item xs={12} mt={4}>
                <VolumeChart />
              </Grid>
            </Grid>
          </Grid>

          {connected && (
            <>
              <WalletBreakdownBox mt={8} key="wallet-breakdown" />
              <LiquidityProvidersBox mt={8} key="liquidity-provider-box" />
            </>
          )}
        </Box>
      </Container>
    </PoolLayout>
  );
};

export default Home;
