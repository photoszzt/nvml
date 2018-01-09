// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) struct CtoBoxInner<Value: CtoSafe>
{
	cto_pool_arc: CtoPoolArc,
	value: Value,
}

impl<Value: CtoSafe> Deref for CtoBoxInner<Value>
{
	type Target = Value;
	
	fn deref(&self) -> &Value
	{
		&self.value
	}
}

impl<Value: CtoSafe> DerefMut for CtoBoxInner<Value>
{
	fn deref_mut(&mut self) -> &mut Value
	{
		&mut self.value
	}
}

impl<Value: CtoSafe> CtoBoxInner<Value>
{
	#[inline(always)]
	fn common_initialization(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.replace(&mut self.cto_pool_arc);
	}
	
	#[inline(always)]
	fn created<InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&mut self, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<(), InitializationError>
	{
		self.common_initialization(cto_pool_arc);
		
		initializer(&mut self.value, cto_pool_arc)
	}
	
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.common_initialization(cto_pool_arc);
		
		self.value.cto_pool_opened(cto_pool_arc)
	}
}
