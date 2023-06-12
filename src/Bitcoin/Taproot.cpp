#include <Data.h>
#include <Taproot.h>

using namespace TW;

TaprootBuilder::TaprootBuilder() {
    builder = TW::Rust::tw_transaction_builder_create();
}

Result<void, Error> TaprootBuilder::AddP2PKHInput(const uint8_t *txid, uint32_t vout, const uint8_t *pubkey, size_t pubkey_len, uint64_t satoshis) {
    auto input = TW::Rust::tw_tx_input_p2pkh_create(txid, vout, pubkey, pubkey_len, satoshis);
	if (input == nullptr) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidInput);
	}

    auto updatedBuilder = TW::Rust::tw_transaction_add_p2pkh_input(builder, input);
	if (updatedBuilder == false) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

	builder = updatedBuilder;
	return Result<void, TaprootBuilderError>::success();
}

Result<void, Error> TaprootBuilder::AddP2WPKHInput(const uint8_t *txid, uint32_t vout, const uint8_t *pubkey, size_t pubkey_len, uint64_t satoshis) {
    auto input = TW::Rust::tw_tx_input_p2wpkh_create(txid, vout, pubkey, pubkey_len, satoshis);
	if (input == nullptr) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidInput);
	}

    auto updatedBuilder = TW::Rust::tw_transaction_add_p2wpkh_input(builder, input);
	if (updatedBuilder == false) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

	builder = updatedBuilder;
	return Result<void, TaprootBuilderError>::success();
}

Result<void, Error> TaprootBuilder::AddP2TRKeyPathInput(const uint8_t *txid, uint32_t vout, const uint8_t *pubkey, size_t pubkey_len, uint64_t satoshis) {
    auto input = TW::Rust::tw_tx_input_p2tr_key_path_create(txid, vout, pubkey, pubkey_len, satoshis);
	if (input == nullptr) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidInput);
	}

    auto updatedBuilder = TW::Rust::tw_transaction_add_p2tr_key_path_input(builder, input);
	if (updatedBuilder == false) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidInput);
	}

	builder = updatedBuilder;
	return Result<void, TaprootBuilderError>::success();
}

Result<void, Error> TaprootBuilder::AddP2PKHOutput(const uint8_t *pubkey, size_t pubkey_len, uint64_t satoshis) {
    auto output = TW::Rust::tw_tx_output_p2pkh_create(pubkey, pubkey_len, satoshis);
	if (output == nullptr) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

    auto updatedBuilder = TW::Rust::tw_transaction_add_p2pkh_output(builder, output);
	if (updatedBuilder == false) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

	builder = updatedBuilder;
	return Result<void, TaprootBuilderError>::success();
}

Result<void, Error> TaprootBuilder::AddP2WPKHOutput(const uint8_t *pubkey, size_t pubkey_len, uint64_t satoshis) {
    auto output = TW::Rust::tw_tx_output_p2wpkh_create(pubkey, pubkey_len, satoshis);
	if (output == nullptr) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

    auto updatedBuilder = TW::Rust::tw_transaction_add_p2wpkh_output(builder, output);
	if (updatedBuilder == false) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

	builder = updatedBuilder;
	return Result<void, TaprootBuilderError>::success();
}

Result<void, Error> TaprootBuilder::AddP2TRKeyPathOutput(const uint8_t *pubkey, size_t pubkey_len, uint64_t satoshis) {
    auto output = TW::Rust::tw_tx_output_p2tr_key_path_create(pubkey, pubkey_len, satoshis);
	if (output == nullptr) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

    auto updatedBuilder = TW::Rust::tw_transaction_add_p2tr_key_path_output(builder, output);
	if (updatedBuilder == false) {
		return Result<void, TaprootBuilderError>::failure(TaprootBuilderError::InvalidOutput);
	}

	builder = updatedBuilder;
	return Result<void, TaprootBuilderError>::success();
}

Result<Data, Error> TaprootBuilder::Sign(const uint8_t *secret_key, size_t secret_key_len) {
    Rust::CByteArrayResultWrapper res = TW::Rust::tw_transaction_sign(builder, secret_key, secret_key_len);
	if (res.isErr()) {
		return Result<Data, TaprootBuilderError>::failure(TaprootBuilderError::FailedToSign);
	}

	return Result<Data, TaprootBuilderError>::success(res.unwrap().data);
}
