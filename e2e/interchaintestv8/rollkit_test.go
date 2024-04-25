package main

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/suite"

	govv1 "cosmossdk.io/api/cosmos/gov/v1"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/testutil"

	"github.com/cosmos/rollkit-ibc/e2e/interchaintest/v8/e2esuite"
)

// RollkitTestSuite is a suite of tests that wraps the TestSuite
// and can provide additional functionality
type RollkitTestSuite struct {
	e2esuite.TestSuite
}

// SetupSuite calls the underlying RollkitTestSuite's SetupSuite method
func (s *RollkitTestSuite) SetupSuite(ctx context.Context) {
	s.TestSuite.SetupSuite(ctx)
}

// TestWithRollkitTestSuite is the boilerplate code that allows the test suite to be run
func TestWithRollkitTestSuite(t *testing.T) {
	suite.Run(t, new(RollkitTestSuite))
}

// TestBasic is an example test function that will be run by the test suite
func (s *RollkitTestSuite) TestBasic() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	// Add your test code here. For example, create a transfer channel between ChainA and ChainB:
	s.Run("StoreContractCode", func() {
		// Submit store contract code proposal on ChainA
		// The contract code should be built in the CI/CD pipeline first
		codeHash, err := s.ChainA.StoreClientContract(ctx, s.UserA.KeyName(), "../../artifacts/rollkit_ibc.wasm")
		s.Require().NoError(err)
		s.Require().NotEmpty(codeHash)

		// vote on the proposal
		err = s.ChainA.VoteOnProposalAllValidators(ctx, "1", cosmos.ProposalVoteYes)
		s.Require().NoError(err)

		// wait for the proposal to pass
		err = testutil.WaitForCondition(time.Second*30, time.Second*5, func() (bool, error) {
			resp, err := e2esuite.GRPCQuery[govv1.QueryProposalResponse](ctx, s.ChainA, &govv1.QueryProposalRequest{ProposalId: 1})
			if err != nil {
				return false, err
			}

			return resp.Proposal.Status == govv1.ProposalStatus_PROPOSAL_STATUS_PASSED, nil
		})
		s.Require().NoError(err)
	})
}
