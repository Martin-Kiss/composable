import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { DEFI_CONFIG } from "@/defi/config";
import { fetchPabloTotalVolume, Range } from "@/defi/subsquid/overview";
import { pipe } from "fp-ts/lib/function";
import * as TE from "fp-ts/TaskEither";
import * as E from "fp-ts/Either";
import { flow } from "fp-ts/function";
import * as A from "fp-ts/ReadonlyArray";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { parseLockedValue } from "@/components/Organisms/overview/parseLockedValue";
import { usePicaPriceDiscovery } from "@/defi/hooks/overview/usePicaPriceDiscovery";

const durationLabels: string[] = [];

export function useVolumeChart(): {
  chartSeries: [number, number][];
  selectedInterval: string;
  setSelectedInterval: Dispatch<SetStateAction<Range>>;
  durationLabels: string[];
  isLoading: boolean;
} {
  const [selectedInterval, setSelectedInterval] = useState<Range>(
    DEFI_CONFIG.swapChartIntervals[0].range as Range
  );
  const hasFetchedTokens = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const picaPrice = usePicaPriceDiscovery();
  const getTokenById = useStore((store) => store.substrateTokens.getTokenById);

  useEffect(() => {
    const task = pipe(
      TE.fromIO(() => setIsLoading(true)),
      TE.chain(fetchPabloTotalVolume(selectedInterval)),
      TE.chainFirst(() => TE.fromIO(() => setIsLoading(false)))
    );
    if (hasFetchedTokens) {
      task().then(
        flow(
          E.match(
            () => setChartSeries([]),
            (totalVolume) => {
              const chart = pipe(
                A.fromArray(totalVolume.pabloTotalVolume),
                A.map((item) => {
                  const date = Date.parse(item.date);

                  const tvl = item.volumes.reduce(
                    parseLockedValue(getTokenById, picaPrice),
                    new BigNumber(0)
                  );

                  return [date, tvl.toNumber()] as const;
                }),
                A.toArray
              );
              setChartSeries(chart as [number, number][]);
            }
          )
        )
      );
    }
  }, [selectedInterval, hasFetchedTokens, getTokenById, picaPrice]);

  return {
    isLoading,
    chartSeries,
    selectedInterval,
    setSelectedInterval,
    durationLabels,
  };
}
