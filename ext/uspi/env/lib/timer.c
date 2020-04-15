//
// timer.c
//
// USPi - An USB driver for Raspberry Pi written in C
// Copyright (C) 2014-2015  R. Stange <rsta2@o2online.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
#include <uspienv/timer.h>
#include <uspienv/bcm2835.h>
#include <uspienv/memio.h>
#include <uspienv/synchronize.h>
#include <uspienv/sysconfig.h>
#include <uspienv/alloc.h>
#include <uspienv/logger.h>
#include <uspienv/debug.h>
#include <uspienv/assert.h>

void DelayLoop (unsigned nCount);
void TimerPollKernelTimers (TTimer *pThis);
void TimerInterruptHandler (void *pParam);
void TimerTuneMsDelay (TTimer *pThis);

#define EnterCritical uspi_EnterCritical
#define LeaveCritical uspi_LeaveCritical

/* don't expose these timer constructor */
static void Timer (TTimer *pThis)
{
	assert (pThis != 0);

	pThis->m_nTicks = 0;
	pThis->m_nTime = 0;
#ifdef ARM_DISABLE_MMU
	pThis->m_nMsDelay = 12500;
#else
	pThis->m_nMsDelay = 350000;
#endif
	pThis->m_nusDelay = pThis->m_nMsDelay / 1000;

	for (unsigned hTimer = 0; hTimer < KERNEL_TIMERS; hTimer++)
	{
		pThis->m_KernelTimer[hTimer].m_pHandler = 0;
	}
}

static boolean TimerInitialize(TTimer *pThis)
{
	assert (pThis != 0);
	assert (!pThis->initialized);

	// XXX. invoke Rust's
	// InterruptSystemConnectIRQ (pThis->m_pInterruptSystem, ARM_IRQ_TIMER3, TimerInterruptHandler, pThis);
	ConnectInterrupt(ARM_IRQ_TIMER3, TimerInterruptHandler, pThis);

	DataMemBarrier ();

	// XXX. Isn't CLO read-only?
	// write32 (ARM_SYSTIMER_CLO, -(30 * CLOCKHZ));	// timer wraps soon, to check for problems

	write32 (ARM_SYSTIMER_C3, read32 (ARM_SYSTIMER_CLO) + CLOCKHZ / HZ);

	// XXX. m_nMsDelay doesn't seem well-tuned
	// TimerTuneMsDelay (pThis);

	DataMemBarrier ();

	pThis->initialized = 1;

	return TRUE;
}

TTimer *TimerGet (void)
{
	static TTimer gTimer;

	if (!gTimer.initialized) {
		Timer(&gTimer);
		TimerInitialize(&gTimer);
	}
	return &gTimer;
}

unsigned TimerGetClockTicks (TTimer *pThis)
{
	assert (pThis != 0);

	DataMemBarrier ();

	unsigned nResult = read32 (ARM_SYSTIMER_CLO);

	DataMemBarrier ();

	return nResult;
}

unsigned TimerGetTicks (TTimer *pThis)
{
	assert (pThis != 0);

	return pThis->m_nTicks;
}

unsigned TimerGetTime (TTimer *pThis)
{
	assert (pThis != 0);

	return pThis->m_nTime;
}

TString *TimerGetTimeString (TTimer *pThis)
{
	assert (pThis != 0);

	EnterCritical ();

	unsigned nTime = pThis->m_nTime;
	unsigned nTicks = pThis->m_nTicks;

	LeaveCritical ();

	if (nTicks == 0)
	{
		return 0;
	}

	unsigned nSecond = nTime % 60;
	nTime /= 60;
	unsigned nMinute = nTime % 60;
	nTime /= 60;
	unsigned nHours = nTime;

	nTicks %= HZ;
#if (HZ != 100)
	nTicks = nTicks * 100 / HZ;
#endif

	TString *pString = malloc (sizeof (TString));
	assert (pString != 0);
	String (pString);

	StringFormat (pString, "%02u:%02u:%02u.%02lu", nHours, nMinute, nSecond, nTicks);

	return pString;
}

unsigned TimerStartKernelTimer (TTimer *pThis, unsigned nDelay, TKernelTimerHandler *pHandler, void *pParam, void *pContext)
{
	assert (pThis != 0);

	EnterCritical ();

	unsigned hTimer;
	for (hTimer = 0; hTimer < KERNEL_TIMERS; hTimer++)
	{
		if (pThis->m_KernelTimer[hTimer].m_pHandler == 0)
		{
			break;
		}
	}

	if (hTimer >= KERNEL_TIMERS)
	{
		LeaveCritical ();
		LogWrite ("timer", 0, "System limit of kernel timers exceeded");
		return 0;
	}

	/* LogWrite ("timer", 0, "Adding handler 0x%08x at %d (tick: %u, delay: %u)", */
  /*           pHandler, hTimer, pThis->m_nTicks, nDelay); */

	assert (pHandler != 0);
	pThis->m_KernelTimer[hTimer].m_pHandler    = pHandler;
	pThis->m_KernelTimer[hTimer].m_nElapsesAt  = pThis->m_nTicks+nDelay;
	pThis->m_KernelTimer[hTimer].m_pParam      = pParam;
	pThis->m_KernelTimer[hTimer].m_pContext    = pContext;

	LeaveCritical ();

	return hTimer+1;
}

void TimerCancelKernelTimer (TTimer *pThis, unsigned hTimer)
{
	assert (pThis != 0);

	assert (1 <= hTimer && hTimer <= KERNEL_TIMERS);
	pThis->m_KernelTimer[hTimer-1].m_pHandler = 0;
}

/*
void TimerMsDelay (TTimer *pThis, unsigned nMilliSeconds)
{
	assert (pThis != 0);

	if (nMilliSeconds > 0)
	{
		unsigned nCycles =  pThis->m_nMsDelay * nMilliSeconds;

		DelayLoop (nCycles);
	}
}

void TimerusDelay (TTimer *pThis, unsigned nMicroSeconds)
{
	assert (pThis != 0);

	if (nMicroSeconds > 0)
	{
		unsigned nCycles =  pThis->m_nusDelay * nMicroSeconds;

		DelayLoop (nCycles);
	}
}

void TimerSimpleMsDelay (unsigned nMilliSeconds)
{
	if (nMilliSeconds > 0)
	{
		TimerSimpleusDelay (nMilliSeconds * 1000);
	}
}

void TimerSimpleusDelay (unsigned nMicroSeconds)
{
	if (nMicroSeconds > 0)
	{
		unsigned nTicks = nMicroSeconds * (CLOCKHZ / 1000000);

		DataMemBarrier ();

		unsigned nStartTicks = read32 (ARM_SYSTIMER_CLO);
		while (read32 (ARM_SYSTIMER_CLO) - nStartTicks < nTicks)
		{
			// do nothing
		}

		DataMemBarrier ();
	}
}
*/

void TimerPollKernelTimers (TTimer *pThis)
{
	assert (pThis != 0);

	EnterCritical ();

	for (unsigned hTimer = 0; hTimer < KERNEL_TIMERS; hTimer++)
	{
		volatile TKernelTimer *pTimer = &pThis->m_KernelTimer[hTimer];

		TKernelTimerHandler *pHandler = pTimer->m_pHandler;
		if (pHandler != 0)
		{
			// LogWrite ("timer", 0, "Handler exists %d", hTimer);
			if ((int) (pTimer->m_nElapsesAt - pThis->m_nTicks) <= 0)
			{
				pTimer->m_pHandler = 0;
				// LogWrite ("timer", 0, "Calling handler 0x%08x with param (0x%08x), context (0x%08x)", pHandler, pTimer->m_pParam, pTimer->m_pContext);

				(*pHandler) (hTimer+1, pTimer->m_pParam, pTimer->m_pContext);
			}
		}
	}

	LeaveCritical ();
}

void TimerInterruptHandler (void *pParam)
{
	TTimer *pThis = (TTimer *) pParam;
	assert (pThis != 0);

	DataMemBarrier ();

	assert (read32 (ARM_SYSTIMER_CS) & (1 << 3));

	/*
	u32 nCompare = read32 (ARM_SYSTIMER_C3) + CLOCKHZ / HZ;
	write32 (ARM_SYSTIMER_C3, nCompare);
	if (nCompare < read32 (ARM_SYSTIMER_CLO))			// time may drift
	{
		nCompare = read32 (ARM_SYSTIMER_CLO) + CLOCKHZ / HZ;
		write32 (ARM_SYSTIMER_C3, nCompare);
	}
	*/
	u32 nCompare = read32 (ARM_SYSTIMER_CLO) + CLOCKHZ / HZ;
	write32 (ARM_SYSTIMER_C3, nCompare);

	write32 (ARM_SYSTIMER_CS, 1 << 3);

	DataMemBarrier ();

	if (++pThis->m_nTicks % HZ == 0)
	{
		pThis->m_nTime++;
	}

	TimerPollKernelTimers (pThis);
}

/*
void TimerTuneMsDelay (TTimer *pThis)
{
	assert (pThis != 0);

	unsigned nTicks = TimerGetTicks (pThis);
	TimerMsDelay (pThis, 1000);
	nTicks = TimerGetTicks (pThis) - nTicks;

	unsigned nFactor = 100 * HZ / nTicks;

	pThis->m_nMsDelay = pThis->m_nMsDelay * nFactor / 100;
	pThis->m_nusDelay = (pThis->m_nMsDelay + 500) / 1000;

	LogWrite ("timer", 0, "SpeedFactor is %u.%02u", nFactor / 100, nFactor % 100);
}
*/

void DelayLoop(unsigned nCycles) {
	volatile register unsigned i;
	for (i = 0; i < nCycles; i++) {
		;
	}
}