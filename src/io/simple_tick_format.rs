use Timestamp;


struct Tick {
    ts: Timestamp,
}


trait ReadTicks {
    fn read_ticks() -> TickIterator;
}


struct TickIterator {

}


