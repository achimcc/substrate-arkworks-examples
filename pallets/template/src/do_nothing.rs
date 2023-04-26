use ark_std::io::Error;
use sp_io::elliptic_curves::do_nothing;

pub fn do_nothing_call() -> Result<(), Error> {
	do_nothing().unwrap();
	Ok(())
}
