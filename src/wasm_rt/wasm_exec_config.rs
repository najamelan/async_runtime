/// The configuration for wich executor that should be used on this thread
//
#[ derive( Debug, Clone, Hash, PartialEq, Eq ) ]
//
pub enum WasmExecConfig
{
	Pool  ,
	Local ,
}


impl Default for WasmExecConfig
{
	fn default() -> Self
	{
		WasmExecConfig::Pool
	}
}