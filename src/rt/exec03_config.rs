/// The configuration for wich executor that should be used on this thread
//
#[ derive( Debug, Clone, Hash, PartialEq, Eq ) ]
//
pub enum Exec03Config
{
	Pool  ,
	Local ,
}


impl Default for Exec03Config
{
	fn default() -> Self
	{
		Exec03Config::Pool
	}
}
