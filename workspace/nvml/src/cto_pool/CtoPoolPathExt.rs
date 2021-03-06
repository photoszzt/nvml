// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// An extension trait to make it easier to use a Path to access a CTO pool.
pub trait CtoPoolPathExt
{
	/// Validate that an existing CTO pool is consistent.
	#[inline(always)]
	fn validate_cto_pool_is_consistent(&self, layout_name: &CStr) -> Result<bool, PmdkError>;
	
	/// Open an existing CTO pool.
	#[inline(always)]
	fn open_cto_pool(&self, layout_name: &CStr) -> Result<*mut PMEMctopool, PmdkError>;
	
	/// Create (and implicitly open) a new CTO pool.
	#[inline(always)]
	fn create_cto_pool(&self, layout_name: &CStr, pool_size: usize, mode: mode_t) -> Result<*mut PMEMctopool, PmdkError>;
}

impl CtoPoolPathExt for Path
{
	#[inline(always)]
	fn validate_cto_pool_is_consistent(&self, layout_name: &CStr) -> Result<bool, PmdkError>
	{
		let result = use_path!(self, pmemcto_check, layout_name.as_ptr());
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => PmdkError::cto("pmemcto_check"),
			illegal @ _ => panic!("pmemcto_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn open_cto_pool(&self, layout_name: &CStr) -> Result<*mut PMEMctopool, PmdkError>
	{
		let result = use_path!(self, pmemcto_open, layout_name.as_ptr());
		
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_open")
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn create_cto_pool(&self, layout_name: &CStr, pool_size: usize, mode: mode_t) -> Result<*mut PMEMctopool, PmdkError>
	{
		let result = use_path!(self, pmemcto_create, layout_name.as_ptr(), pool_size, mode);
		
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_create")
		}
		else
		{
			Ok(result)
		}
	}
}
