/// The configuration for wich executor that should be used on this thread
//
#[ derive( Debug, Clone, Hash, PartialEq, Eq ) ]
//
pub enum RtConfig
{
	Pool  ,
	Local ,
}


impl Default for RtConfig
{
	fn default() -> Self
	{
		RtConfig::Pool
	}
}
