#![macro_use]


/**
 * Define a callback using a React-like syntax.
 */
#[macro_export]
macro_rules! callback {
    { | $( $cbvar:ident: $cbty:ty ),* | => $block:block, [ $( $depvar:ident ),* ] } => {
        {
            $( let $depvar = $depvar.clone(); )*

            Callback::from(move | $( $cbvar: $cbty ),* | $block)
        }
    }
}
