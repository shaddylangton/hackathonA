// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract EventContract {
    struct Attendee {
        bool isRegistered;
        uint tokenCount;
        mapping(uint => bool) tokens;
    }
    
    mapping(address => Attendee) public attendees;
    
    function registerAttendee() public {
        attendees[msg.sender].isRegistered = true;
    }
    
    function issueTokens(uint tokenCount) public {
        require(attendees[msg.sender].isRegistered, "Attendee is not registered");
        attendees[msg.sender].tokenCount += tokenCount;
        for (uint i = 0; i < tokenCount; i++) {
            attendees[msg.sender].tokens[i] = true;
        }
    }
    
    function verifyAttendance(address attendee, uint tokenId) public view returns (bool) {
        return attendees[attendee].tokens[tokenId];
    }
}
